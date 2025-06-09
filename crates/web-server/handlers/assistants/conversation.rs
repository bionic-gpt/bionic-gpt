use super::super::console::process_chats;
use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries;
use db::Pool;
use db::{authz, ModelType};
use integrations::ToolScope;
use llm_proxy::UserConfig;
use openai_api::BionicToolDefinition;
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

    // Process chats to get chat_history and pending_chat_state
    let (chat_history, pending_chat_state) = process_chats(&transaction, chats).await?;

    let has_pending_chat = !matches!(
        &pending_chat_state,
        web_pages::console::PendingChatState::None
    );

    tracing::debug!(
        "History items = {} and pending chat? {}",
        chat_history.len(),
        has_pending_chat
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

    let available_tools: Vec<BionicToolDefinition> =
        integrations::get_tools(ToolScope::UserSelectable);

    let html = web_pages::assistants::conversation::page(
        team_id,
        rbac,
        chat_history,
        pending_chat_state,
        prompt,
        conversation_id,
        is_tts_disabled,
        capabilities,
        enabled_tools,
        available_tools,
    );

    Ok(Html(html))
}
