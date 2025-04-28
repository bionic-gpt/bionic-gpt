use super::super::{CustomError, Jwt};
use crate::user_config::UserConfig;
use axum::extract::Extension;
use axum::response::Html;
use db::queries::{capabilities, chats, chats_chunks, models, prompts};
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
    user_config: UserConfig,
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

        // Set tool_calls only if this is the last chat and tool_calls is Some
        let tool_calls = if index == last_chat_index && chat.tool_calls.is_some() {
            // Parse the tool_calls JSON string into a Vec<ToolCall>
            match from_str::<Vec<ToolCall>>(&chat.tool_calls.clone().unwrap()) {
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

    let prompt_id = if let Some(default_prompt) = user_config.default_prompt {
        default_prompt
    } else {
        prompts.first().unwrap().id
    };

    let prompt = prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let capabilities = capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;

    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    let html = console::conversation::page(
        team_id,
        rbac,
        chats_with_chunks,
        prompts,
        prompt,
        conversation_id,
        lock_console,
        is_tts_disabled,
        capabilities,
        enabled_tools,
    );

    Ok(Html(html))
}
