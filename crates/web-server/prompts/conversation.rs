use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries::{capabilities, chats, chats_chunks, models, prompts};
use db::{authz, ModelType};
use db::{Chat, Pool};
use llm_proxy::UserConfig;
use openai_api::ToolCall;
use serde_json::from_str;
use web_pages::console::ChatWithChunks;
use web_pages::routes::prompts::Conversation;

pub async fn conversation(
    Conversation {
        team_id,
        conversation_id,
        prompt_id,
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

    let mut chat_history: Vec<ChatWithChunks> = Vec::new();
    let pending_chat: Option<Chat> = None;

    // Get the last chat index for comparison
    let last_chat_index = chats.len().saturating_sub(1);

    for (index, chat) in chats.into_iter().enumerate() {
        // Get all chunks for each chat
        let chunks_chats = chats_chunks::chunks_chats()
            .bind(&transaction, &chat.id)
            .all()
            .await?;

        // Set tool_calls only if this is the last chat and tool_calls is Some
        let tool_calls = if index == last_chat_index && chat.tool_calls.is_some() {
            // Parse the tool_calls JSON string into a Vec<ToolCall>
            from_str::<Vec<ToolCall>>(&chat.tool_calls.clone().unwrap()).ok()
        } else {
            None
        };

        let chat_with_chunks = ChatWithChunks {
            chat,
            chunks: chunks_chats,
            tool_calls,
        };
        chat_history.push(chat_with_chunks);
    }

    let prompt = prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let is_tts_disabled = models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    let capabilities = capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;
    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    let available_tools: Vec<(String, String)> =
        integrations::get_user_selectable_tools_for_chat_ui();

    let html = web_pages::prompts::conversation::page(
        team_id,
        rbac,
        chat_history,
        pending_chat,
        prompt,
        conversation_id,
        is_tts_disabled,
        capabilities,
        enabled_tools,
        available_tools,
    );

    Ok(Html(html))
}
