use crate::{CustomError, Jwt};
use agent_runtime::user_config::UserConfig;
use axum::extract::Extension;
use axum::response::Html;
use db::queries;
use db::Pool;
use db::{authz, ModelType};
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

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let chats = queries::chats::chats()
        .bind(&transaction, &conversation_id)
        .all()
        .await?;

    let project_id = queries::conversations::conversation_project()
        .bind(&transaction, &conversation_id)
        .one()
        .await?
        .project_id;

    let is_tts_disabled = queries::models::models()
        .bind(&transaction, &ModelType::TextToSpeech)
        .all()
        .await?
        .is_empty();

    // Process chats to get chat_history and pending_chat_state
    let (chat_history, pending_chat_state) =
        super::utils::process_chats(&transaction, chats).await?;

    let prompts = queries::models::all_models()
        .bind(&transaction)
        .all()
        .await?;

    if prompts.is_empty() {
        return Err(CustomError::FaultySetup(
            "No model prompts configured".to_string(),
        ));
    }

    let model_id = if let Some(default_model) = user_config.default_model {
        default_model
    } else {
        prompts.first().unwrap().id
    };

    let prompt = queries::models::model_config()
        .bind(&transaction, &model_id)
        .one()
        .await;

    let prompt = if let Ok(prompt) = prompt {
        prompt
    } else {
        let id = prompts.first().unwrap().id;
        queries::models::model_config()
            .bind(&transaction, &id)
            .one()
            .await?
    };

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.id)
        .all()
        .await?;
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
        project_id,
    );

    Ok(Html(html))
}
