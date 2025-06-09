use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::queries;
use db::Pool;
use db::{authz, ModelType};
use integrations;
use integrations::ToolScope;
use llm_proxy::user_config::UserConfig;
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

    let chats = queries::chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    let is_tts_disabled = queries::models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    // Process chats to get chat_history and pending_chat_state
    let (chat_history, pending_chat_state) =
        super::utils::process_chats(&transaction, chats).await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id, &db::PromptType::Model)
        .all()
        .await?;

    let prompt_id = if let Some(default_prompt) = user_config.default_prompt {
        default_prompt
    } else {
        prompts.first().unwrap().id
    };

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id)
        .one()
        .await?;

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;
    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    let available_tools = integrations::get_tools(ToolScope::UserSelectable);

    let html = console::conversation::page(
        team_id,
        rbac,
        chat_history,
        pending_chat_state,
        prompts,
        prompt,
        conversation_id,
        is_tts_disabled,
        capabilities,
        enabled_tools,
        available_tools,
    );

    Ok(Html(html))
}
