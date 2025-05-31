use crate::CustomError;
use db::queries::{chats::Chat, chats_chunks};
use db::{ChatRole, ChatStatus, Transaction};
use openai_api::ToolCall;
use serde_json::from_str;
use web_pages::console::{ChatWithChunks, PendingChat, PendingChatState};

/// Process a list of chats to create chat history and identify pending chats.
///
/// This function takes a list of chats and processes them to:
/// 1. Separate pending chats by role (Tool vs User)
/// 2. If pending Tool chats exist, return PendingChatState::PendingToolChats
/// 3. Else if pending User chat exists, return PendingChatState::PendingUserChat
/// 4. Else return PendingChatState::None
/// 5. For each chat in chat_history, fetch its chunks
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

    // If there are no chats, return empty results
    if chats.is_empty() {
        return Ok((chat_history, PendingChatState::None));
    }

    // Get the last chat ID from the original input list
    let last_chat_id = chats.last().map(|chat| chat.id).unwrap_or(0);

    // Separate pending and non-pending chats
    let (pending_chats, non_pending_chats): (Vec<Chat>, Vec<Chat>) = chats
        .clone()
        .into_iter()
        .partition(|chat| chat.status == ChatStatus::Pending);

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
