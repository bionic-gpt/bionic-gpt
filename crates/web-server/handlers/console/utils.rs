use crate::CustomError;
use db::queries::{chats::Chat, chats_chunks};
use db::{ChatRole, ChatStatus, Transaction};
use openai_api::ToolCall;
use serde_json::from_str;
use time::{Duration, OffsetDateTime};
use web_pages::console::{ChatWithChunks, PendingChat, PendingChatState};

pub fn determine_pending_chat_state(chats: Vec<Chat>) -> (Vec<Chat>, PendingChatState) {
    if chats.is_empty() {
        return (Vec::new(), PendingChatState::None);
    }

    tracing::debug!("Got {} chats", chats.len());

    let last_chat_id = chats.last().map(|chat| chat.id).unwrap_or(0);

    // Collect non-pending chats for return
    let non_pending: Vec<Chat> = chats
        .iter()
        .filter(|&chat| !(chat.status == ChatStatus::Pending))
        .cloned()
        .collect();

    // Walk tail in reverse to find consecutive recent pending tool/user
    let mut tail_pending_tool_chats = Vec::new();
    for chat in chats.iter().rev() {
        if chat.status == ChatStatus::Pending {
            match chat.role {
                ChatRole::Tool => tail_pending_tool_chats.push(chat.clone()),
                ChatRole::User => {
                    if chat.id == last_chat_id {
                        let tool_calls = chat
                            .tool_calls
                            .as_ref()
                            .and_then(|s| from_str::<Vec<ToolCall>>(s).ok());

                        return (
                            non_pending,
                            PendingChatState::PendingUserChat(Box::new(PendingChat {
                                chat: chat.clone(),
                                tool_calls,
                            })),
                        );
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    if !tail_pending_tool_chats.is_empty() {
        tail_pending_tool_chats.reverse(); // restore order
        return (
            non_pending,
            PendingChatState::PendingToolChats(tail_pending_tool_chats, last_chat_id),
        );
    }

    (non_pending, PendingChatState::None)
}

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

    tracing::debug!(
        "Shall we call the model {}",
        pending_chat_state.shall_we_call_the_model()
    );
    tracing::debug!("{:?}", pending_chat_state);

    Ok((chat_history, pending_chat_state))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, OffsetDateTime};

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

        assert!(pending_chat_state.shall_we_call_the_model());

        // Should have pending chats since some are recent
        assert_ne!(
            pending_chat_state,
            PendingChatState::None,
            "Should have pending chats"
        );
        assert_eq!(
            non_pending_chats.len(),
            4,
            "Four old chats should be non-pending"
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
