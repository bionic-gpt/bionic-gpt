use crate::CustomError;
use db::queries::{chats::Chat, chats_chunks};
use db::{ChatStatus, Transaction};
use openai_api::ToolCall;
use serde_json::from_str;
use web_pages::console::{ChatWithChunks, PendingChat};

/// Process a list of chats to create chat history and identify pending chats.
///
/// This function takes a list of chats and processes them to:
/// 1. If the last chat has a status of ChatStatus::Pending, set it as pending_chat and exclude it from chat_history
/// 2. Otherwise, set pending_chat to None and include all chats in chat_history
/// 3. For each chat in chat_history, fetch its chunks and set tool_calls if it's the last chat
///
/// # Arguments
/// * `transaction` - The database transaction
/// * `chats` - The list of chats to process
///
/// # Returns
/// A tuple containing:
/// * `chat_history` - A vector of ChatWithChunks for all non-pending chats
/// * `pending_chat` - An Option containing the pending chat if the last chat is pending, otherwise None
pub async fn process_chats(
    transaction: &Transaction<'_>,
    chats: Vec<Chat>,
) -> Result<(Vec<ChatWithChunks>, Option<PendingChat>), CustomError> {
    let mut chat_history: Vec<ChatWithChunks> = Vec::new();
    let mut pending_chat: Option<PendingChat> = None;

    // If there are no chats, return empty results
    if chats.is_empty() {
        return Ok((chat_history, pending_chat));
    }

    // Check if the last chat is pending
    let last_chat = chats.last().unwrap();
    let is_last_chat_pending = last_chat.status == ChatStatus::Pending;

    // Process all chats except possibly the last one if it's pending
    let chats_to_process = if is_last_chat_pending {
        // Set the last chat as pending_chat
        let tool_calls = if last_chat.tool_calls.is_some() {
            // Parse the tool_calls JSON string into a Vec<ToolCall>
            from_str::<Vec<ToolCall>>(&last_chat.tool_calls.clone().unwrap()).ok()
        } else {
            None
        };

        pending_chat = Some(PendingChat {
            chat: last_chat.clone(),
            tool_calls,
        });
        // Process all chats except the last one
        &chats[0..chats.len() - 1]
    } else {
        // Process all chats
        &chats[..]
    };

    // Process the chats for chat_history
    for chat in chats_to_process.iter() {
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

    Ok((chat_history, pending_chat))
}
