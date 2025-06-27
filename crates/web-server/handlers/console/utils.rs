use crate::CustomError;
use db::queries::{chats::Chat, chats_chunks};
use db::{ChatRole, ChatStatus, Transaction};
use openai_api::ToolCall;
use serde_json::from_str;
use time::{Duration, OffsetDateTime};
use web_pages::console::{ChatWithChunks, PendingChat, PendingChatState};

/// Determine the pending chat state from a list of chats.
///
/// This function processes chats to identify which ones should be considered pending:
/// 1. Separate pending chats by role (Tool vs User), excluding chats older than 5 seconds
/// 2. If pending Tool chats exist, return PendingChatState::PendingToolChats
/// 3. Else if pending User chat exists, return PendingChatState::PendingUserChat
/// 4. Else return PendingChatState::None
///
/// Note: Chats that are more than 5 seconds old (based on created_at) will never be
/// considered pending, regardless of their database status.
///
/// # Arguments
/// * `chats` - The list of chats to process
///
/// # Returns
/// A tuple containing:
/// * `non_pending_chats` - A vector of chats that should be treated as non-pending
/// * `pending_chat_state` - The pending chat state (Tool chats, User chat, or None)
pub fn determine_pending_chat_state(chats: Vec<Chat>) -> (Vec<Chat>, PendingChatState) {
    // If there are no chats, return empty results
    if chats.is_empty() {
        return (Vec::new(), PendingChatState::None);
    }

    // Get the last chat ID from the original input list
    let last_chat_id = chats.last().map(|chat| chat.id).unwrap_or(0);

    // Separate pending and non-pending chats
    // A chat is only considered pending if:
    // 1. Its status is Pending AND
    // 2. It was created within the last 5 seconds
    let (pending_chats, non_pending_chats): (Vec<Chat>, Vec<Chat>) = chats
        .into_iter()
        .partition(|chat| chat.status == ChatStatus::Pending && !is_chat_too_old_for_pending(chat));

    // Determine pending chat state based on priority
    let pending_chat_state = if !pending_chats.is_empty() {
        // Separate pending chats by role
        let (tool_chats, user_chats): (Vec<Chat>, Vec<Chat>) = pending_chats
            .into_iter()
            .partition(|chat| chat.role == ChatRole::Tool);

        if !tool_chats.is_empty() {
            // Priority 1: Show pending Tool chats
            PendingChatState::PendingToolChats(tool_chats, last_chat_id)
        } else if !user_chats.is_empty() {
            // Priority 2: Show last pending User chat
            let last_user_chat = user_chats.into_iter().next_back().unwrap();
            let tool_calls = if last_user_chat.tool_calls.is_some() {
                // Parse the tool_calls JSON string into a Vec<ToolCall>
                from_str::<Vec<ToolCall>>(&last_user_chat.tool_calls.clone().unwrap()).ok()
            } else {
                None
            };

            PendingChatState::PendingUserChat(Box::new(PendingChat {
                chat: last_user_chat,
                tool_calls,
            }))
        } else {
            // No Tool or User chats pending
            PendingChatState::None
        }
    } else {
        // No pending chats at all
        PendingChatState::None
    };

    (non_pending_chats, pending_chat_state)
}

/// Process a list of chats to create chat history and identify pending chats.
///
/// This function takes a list of chats and processes them to:
/// 1. Determine pending chat state using determine_pending_chat_state()
/// 2. For each non-pending chat, fetch its chunks from the database
///
/// # Arguments
/// * `transaction` - The database transaction
/// * `chats` - The list of chats to process
///
/// # Returns
/// A tuple containing:
/// * `chat_history` - A vector of ChatWithChunks for all non-pending chats
/// * `pending_chat_state` - The pending chat state (Tool chats, User chat, or None)
pub async fn process_chats(
    transaction: &Transaction<'_>,
    chats: Vec<Chat>,
) -> Result<(Vec<ChatWithChunks>, PendingChatState), CustomError> {
    let mut chat_history: Vec<ChatWithChunks> = Vec::new();

    // Determine pending state and get non-pending chats
    let (non_pending_chats, pending_chat_state) = determine_pending_chat_state(chats);

    // Process non-pending chats for chat_history
    for chat in non_pending_chats.iter() {
        // Get all chunks for each chat
        let chunks_chats = chats_chunks::chunks_chats()
            .bind(transaction, &chat.id)
            .all()
            .await?;

        let chat_with_chunks = ChatWithChunks {
            chat: chat.clone(),
            chunks: chunks_chats,
        };
        chat_history.push(chat_with_chunks);
    }

    Ok((chat_history, pending_chat_state))
}

/// Helper function to check if a chat is too old to be considered pending.
/// Chats older than 5 seconds (based on created_at) should not be pending.
fn is_chat_too_old_for_pending(chat: &Chat) -> bool {
    let now = OffsetDateTime::now_utc();
    let five_seconds_ago = now - Duration::seconds(5);
    chat.created_at < five_seconds_ago
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    // Helper function to create a mock Chat for testing
    fn create_mock_chat(
        id: i32,
        status: ChatStatus,
        role: ChatRole,
        created_at: OffsetDateTime,
    ) -> Chat {
        Chat {
            id,
            conversation_id: 49,
            content: Some("Test content".to_string()),
            role,
            tool_call_id: None,
            tool_calls: None,
            prompt_id: 1,
            model_name: "test-model".to_string(),
            status,
            attachments: None,
            created_at,
            updated_at: created_at,
        }
    }

    #[tokio::test]
    async fn test_process_chats_with_recent_pending_chats() {
        // Test with recent pending chats that should remain pending
        let now = OffsetDateTime::now_utc();

        let chats = vec![
            // Recent pending Tool chat (should remain pending)
            create_mock_chat(
                1,
                ChatStatus::Success,
                ChatRole::User,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                2,
                ChatStatus::Success,
                ChatRole::Assistant,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                3,
                ChatStatus::Success,
                ChatRole::Tool,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                4,
                ChatStatus::Success,
                ChatRole::Assistant,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                5,
                ChatStatus::Pending,
                ChatRole::Tool,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                6,
                ChatStatus::Pending,
                ChatRole::Tool,
                now - Duration::seconds(1),
            ),
            create_mock_chat(
                7,
                ChatStatus::Pending,
                ChatRole::Tool,
                now - Duration::seconds(1),
            ),
        ];

        // Test the pending state determination logic
        let (non_pending_chats, pending_chat_state) = determine_pending_chat_state(chats);

        // Should have pending chats since some are recent
        assert_ne!(
            pending_chat_state,
            PendingChatState::None,
            "Should have pending chats"
        );
        assert_eq!(
            non_pending_chats.len(),
            4,
            "Four old chat should be non-pending"
        );

        // Verify the old chat is in non-pending
        let non_pending_ids: Vec<i32> = non_pending_chats.iter().map(|chat| chat.id).collect();
        assert_eq!(
            non_pending_ids,
            vec![1, 2, 3, 4],
            "Old chat should be non-pending"
        );

        // Verify we have the correct pending state (should be PendingToolChats since Tool has priority)
        match pending_chat_state {
            PendingChatState::PendingToolChats(tool_chats, _) => {
                assert_eq!(tool_chats.len(), 3, "Should have three pending tool chats");
                assert_eq!(tool_chats[0].id, 5, "Should be the recent tool chat");
            }
            _ => panic!("Expected PendingToolChats state"),
        }
    }
}
