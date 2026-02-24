use agent_runtime::user_config::UserConfig;
use tool_runtime::ToolScope;

use crate::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries;
use db::Pool;
use tool_runtime;
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

    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    let prompts = queries::prompts::prompts()
        .bind(&transaction, &team_id_num, &db::PromptType::Model)
        .all()
        .await?;

    let prompt_id = if let Some(default_prompt) = user_config.default_prompt {
        default_prompt
    } else {
        prompts.first().unwrap().id
    };

    let prompt = queries::prompts::prompt()
        .bind(&transaction, &prompt_id, &team_id_num)
        .one()
        .await;

    let prompt = if let Ok(prompt) = prompt {
        prompt
    } else {
        let id = prompts.first().unwrap().id;
        queries::prompts::prompt()
            .bind(&transaction, &id, &team_id_num)
            .one()
            .await?
    };

    let capabilities = queries::capabilities::get_model_capabilities()
        .bind(&transaction, &prompt.model_id)
        .all()
        .await?;
    let enabled_tools = user_config.enabled_tools.unwrap_or_default();

    // Get available tools from the integrations crate
    let available_tools =
        tool_runtime::get_tools_with_system_openapi(&pool, ToolScope::UserSelectable).await;

    let html = console::page::new_conversation(
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
