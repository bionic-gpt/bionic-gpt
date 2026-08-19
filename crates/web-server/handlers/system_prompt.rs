use crate::{CustomError, Jwt};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse},
    Router,
};
use axum_extra::routing::RouterExt;
use db::{authz, queries, Pool};
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;
use web_pages::routes::system_prompt::{Index, Update};
use web_pages::system_prompt::page::{
    IntegrationFunctionPreview, PromptSizePreview, SystemPromptPageData, ToolPreview,
};

pub fn routes() -> Router {
    Router::new().typed_get(loader).typed_post(update_action)
}

pub async fn loader(
    Index { team_id }: Index,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
) -> Result<Html<String>, CustomError> {
    let sub = current_user.sub.clone();
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    let setting = queries::runtime_settings::default_system_prompt()
        .bind(&transaction)
        .one()
        .await?;
    let skill_summaries = queries::skills::visible_skill_summaries()
        .bind(&transaction)
        .all()
        .await?;
    let vfs_preview = tool_runtime::builtin_tools::bashkit::preview_vfs_tree(&skill_summaries);
    let runtime_additions =
        tool_runtime::skills::available_skills_prompt_section_with_custom(skill_summaries);

    let tool_definitions =
        tool_runtime::get_chat_tools_user_selected_with_system_openapi(&pool).await;
    let prompt_size_preview = build_prompt_size_preview(
        &setting.value,
        runtime_additions.as_deref(),
        &tool_definitions,
    );
    let tools_preview = build_tool_previews(tool_definitions);
    let integration_functions_preview =
        match tool_runtime::builtin_tools::monty::preview_integration_functions(
            &pool,
            &sub,
            team_id_num,
        )
        .await
        {
            Ok(value) => build_integration_function_previews(value),
            Err(err) => vec![IntegrationFunctionPreview {
                path: "".to_string(),
                integration: "preview_error".to_string(),
                operation: "Failed to preview integration functions".to_string(),
                description: err,
                parameters: Vec::new(),
            }],
        };

    let html = web_pages::system_prompt::page::page(
        team_id,
        rbac,
        SystemPromptPageData {
            setting,
            runtime_additions,
            prompt_size_preview,
            tools_preview,
            integration_functions_preview,
            vfs_preview,
        },
    );

    Ok(Html(html))
}

fn build_prompt_size_preview(
    default_prompt: &str,
    runtime_additions: Option<&str>,
    tools: &[tool_runtime::ToolDefinition],
) -> PromptSizePreview {
    let runtime_additions = runtime_additions.unwrap_or_default();
    let combined_system_message =
        combine_non_empty_sections([default_prompt, runtime_additions].into_iter());
    let tool_metadata = serde_json::to_string(tools).unwrap_or_default();

    let default_prompt_tokens = estimate_text_tokens(default_prompt);
    let runtime_additions_tokens = estimate_text_tokens(runtime_additions);
    let combined_system_message_tokens = estimate_text_tokens(&combined_system_message);
    let tool_metadata_tokens = estimate_text_tokens(&tool_metadata);

    PromptSizePreview {
        default_prompt_tokens,
        runtime_additions_tokens,
        combined_system_message_tokens,
        tool_metadata_tokens,
        total_foundation_tokens: combined_system_message_tokens + tool_metadata_tokens,
        tool_count: tools.len(),
    }
}

fn estimate_text_tokens(text: &str) -> i32 {
    if text.trim().is_empty() {
        return 0;
    }

    tool_runtime::token_count::token_count_from_string(text)
}

fn combine_non_empty_sections<'a>(sections: impl Iterator<Item = &'a str>) -> String {
    sections
        .filter(|section| !section.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn build_tool_previews(tools: Vec<tool_runtime::ToolDefinition>) -> Vec<ToolPreview> {
    let mut previews = tools
        .into_iter()
        .map(|tool| ToolPreview {
            name: tool.name,
            description: tool.description,
            parameters: parameter_names(&tool.parameters),
        })
        .collect::<Vec<_>>();
    previews.sort_by(|left, right| left.name.cmp(&right.name));
    previews
}

fn build_integration_function_previews(value: Value) -> Vec<IntegrationFunctionPreview> {
    let Some(items) = value.as_array() else {
        return Vec::new();
    };

    let mut previews = items
        .iter()
        .map(|item| IntegrationFunctionPreview {
            path: item
                .get("path")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            integration: item
                .get("integration")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            operation: item
                .get("operation")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            description: item
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            parameters: parameter_names(item.get("parameters").unwrap_or(&Value::Null)),
        })
        .collect::<Vec<_>>();
    previews.sort_by(|left, right| left.path.cmp(&right.path));
    previews
}

fn parameter_names(parameters: &Value) -> Vec<String> {
    let mut names = parameters
        .get("properties")
        .and_then(Value::as_object)
        .map(|properties| properties.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_names_are_sorted_from_json_schema_properties() {
        let parameters = serde_json::json!({
            "type": "object",
            "properties": {
                "body": {"type": "string"},
                "to": {"type": "string"},
                "subject": {"type": "string"}
            }
        });

        assert_eq!(
            parameter_names(&parameters),
            vec!["body".to_string(), "subject".to_string(), "to".to_string()]
        );
    }

    #[test]
    fn integration_function_preview_extracts_display_fields() {
        let previews = build_integration_function_previews(serde_json::json!([
            {
                "path": "toolbox.integrations.email.listemails",
                "integration": "email",
                "operation": "listemails",
                "description": "List recent email messages",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "limit": {"type": "integer"}
                    }
                }
            }
        ]));

        assert_eq!(previews.len(), 1);
        assert_eq!(previews[0].operation, "listemails");
        assert_eq!(previews[0].parameters, vec!["limit".to_string()]);
    }

    #[test]
    fn prompt_size_preview_includes_prompt_runtime_and_tools() {
        let tools = vec![tool_runtime::ToolDefinition {
            name: "open_url".to_string(),
            description: "Fetch a URL".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {"type": "string"}
                }
            }),
        }];

        let preview = build_prompt_size_preview(
            "You are helpful.",
            Some("Use available skills when relevant."),
            &tools,
        );

        assert!(preview.default_prompt_tokens > 0);
        assert!(preview.runtime_additions_tokens > 0);
        assert!(preview.combined_system_message_tokens >= preview.default_prompt_tokens);
        assert!(preview.tool_metadata_tokens > 0);
        assert_eq!(preview.tool_count, 1);
        assert_eq!(
            preview.total_foundation_tokens,
            preview.combined_system_message_tokens + preview.tool_metadata_tokens
        );
    }

    #[test]
    fn prompt_size_preview_handles_empty_runtime_additions() {
        let preview = build_prompt_size_preview("System prompt", None, &[]);

        assert!(preview.default_prompt_tokens > 0);
        assert_eq!(preview.runtime_additions_tokens, 0);
        assert_eq!(
            preview.combined_system_message_tokens,
            preview.default_prompt_tokens
        );
        assert_eq!(preview.tool_count, 0);
    }
}

#[derive(Deserialize, Validate, Default, Debug)]
pub struct SystemPromptForm {
    #[validate(length(min = 1, message = "The system prompt is mandatory"))]
    pub value: String,
}

pub async fn update_action(
    Update { team_id }: Update,
    current_user: Jwt,
    Extension(pool): Extension<Pool>,
    Form(form): Form<SystemPromptForm>,
) -> Result<impl IntoResponse, CustomError> {
    let mut client = pool.get().await?;
    let transaction = client.transaction().await?;

    let (rbac, _team_id_num) =
        authz::get_permisisons(&transaction, &current_user.into(), &team_id).await?;

    if !rbac.is_sys_admin {
        return Err(CustomError::Authorization);
    }

    if form.validate().is_err() {
        return Ok(crate::layout::redirect_and_snackbar(
            &web_pages::routes::system_prompt::Index { team_id }.to_string(),
            "System Prompt Validation Error",
        )
        .into_response());
    }

    queries::runtime_settings::update_default_system_prompt()
        .bind(&transaction, &form.value)
        .await?;
    transaction.commit().await?;

    Ok(crate::layout::redirect_and_snackbar(
        &web_pages::routes::system_prompt::Index { team_id }.to_string(),
        "System Prompt Updated",
    )
    .into_response())
}
