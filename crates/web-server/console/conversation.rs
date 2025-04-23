use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries::{chats, chats_chunks, models, prompts};
use db::Pool;
use db::{authz, ModelType};
use openai_api::ToolCall;
use serde_json::from_str;
use web_pages::console::ChatWithChunks;
use web_pages::{console, routes::console::Conversation};

pub async fn conversation(
    Conversation {
        team_id,
        conversation_id,
    }: Conversation,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

    let chats = chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    let is_tts_disabled = models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    let mut chats_with_chunks: Vec<ChatWithChunks> = Vec::new();
    let mut lock_console = false;

    // Get the last chat index for comparison
    let last_chat_index = chats.len().saturating_sub(1);

    for (index, chat) in chats.into_iter().enumerate() {
        // If any chat has not had a response yet, lock the console
        lock_console = chat.response.is_none();

        // Get all chunks for each chat
        let chunks_chats = chats_chunks::chunks_chats()
            .bind(&transaction, &chat.id)
            .all()
            .await?;

        // Set tool_calls only if this is the last chat and function_call is Some
        let tool_calls = if index == last_chat_index && chat.function_call.is_some() {
            // Parse the function_call JSON string into a Vec<ToolCall>
            match from_str::<Vec<ToolCall>>(&chat.function_call.clone().unwrap()) {
                Ok(calls) => {
                    lock_console = true;
                    Some(calls)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        let chat_with_chunks = ChatWithChunks {
            chat,
            chunks: chunks_chats,
            tool_calls,
        };
        chats_with_chunks.push(chat_with_chunks);
    }

    let prompts = prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Model)
        .all()
        .await?;

    let prompt = prompts::prompt()
        .bind(&transaction, &prompts.first().unwrap().id, &team_id)
        .one()
        .await?;

    let html = console::conversation::page(
        team_id,
        rbac,
        chats_with_chunks,
        prompts,
        prompt,
        conversation_id,
        lock_console,
        is_tts_disabled,
    );

    Ok(Html(html))
}
