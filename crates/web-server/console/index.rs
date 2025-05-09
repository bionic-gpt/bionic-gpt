use llm_proxy::user_config::UserConfig;

use super::super::{CustomError, Jwt};
use axum::extract::Extension;
use axum::response::Html;
use db::authz;
use db::queries::{capabilities, prompts};
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

    // Get available tools from the integrations crate
    let available_tools: Vec<(String, String)> = integrations::get_all_available_tools_for_chat()
        .iter()
        .map(|tool| {
            let tool_def = tool.function.description.clone().unwrap_or("".to_string());
            let tool_id = tool.function.name.clone();

            // Use the tool ID as the display name
            // This keeps the display name in one place only
            (tool_id, tool_def)
        })
        .collect();

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
