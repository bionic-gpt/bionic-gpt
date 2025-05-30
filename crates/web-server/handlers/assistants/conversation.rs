use super::super::console::process_chats;
use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries;
use db::Pool;
use db::{authz, ModelType};
use llm_proxy::UserConfig;
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

    let chats = queries::chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    // Process chats to get chat_history and pending_chat
    let (chat_history, pending_chat) = process_chats(&transaction, chats).await?;

    tracing::debug!(
        "History items = {} and pending chat? {}",
        chat_history.len(),
        pending_chat.is_some()
    );

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let is_tts_disabled = queries::models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;
    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    let available_tools: Vec<(String, String)> =
        integrations::get_user_selectable_tools_for_chat_ui();

    let html = web_pages::assistants::conversation::page(
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
