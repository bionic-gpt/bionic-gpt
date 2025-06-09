use integrations::ToolScope;
use llm_proxy::user_config::UserConfig;

use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use integrations;
use web_pages::console;
use web_pages::routes::console::Index;

pub async fn index(
    Index { team_id }: Index,
    current_user: Jwt,
    user_config: UserConfig,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let rbac = authz::get_permissions(&transaction, &current_user.into(), team_id).await?;

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
        .await;

    let prompt = if let Ok(prompt) = prompt {
        prompt
    } else {
        let id = prompts.first().unwrap().id;
        queries::prompts::prompt()
            .bind(&transaction, &id, &team_id)
            .one()
            .await?
    };

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;
    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    // Get available tools from the integrations crate
    let available_tools = integrations::get_tools(ToolScope::UserSelectable);

    let html = console::index::new_conversation(
        team_id,
        prompts,
        prompt,
        rbac,
        capabilities,
        enabled_tools,
        available_tools,
    );

    Ok(Html(html))
}
