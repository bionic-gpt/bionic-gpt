// Import the tool trait and time date tool
use crate::builtin_tools;
use crate::system_tool_sources::get_system_openapi_tool_definitions;
use crate::types::ToolDefinition;
use db::Pool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ToolScope {
    UserSelectable,
    DocumentIntelligence,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IntegrationTool {
    pub title: String,
    pub scope: ToolScope,
    pub definitions: Vec<ToolDefinition>,
    pub definitions_json: String,
}

fn to_definitions_json(definitions: &[ToolDefinition]) -> String {
    serde_json::to_string_pretty(definitions).expect("Failed to serialize tool definitions to JSON")
}

fn integration_tool(
    title: &str,
    scope: ToolScope,
    definitions: Vec<ToolDefinition>,
) -> IntegrationTool {
    let definitions_json = to_definitions_json(&definitions);
    IntegrationTool {
        title: title.into(),
        scope,
        definitions,
        definitions_json,
    }
}

pub fn get_integrations(scope: Option<ToolScope>) -> Vec<IntegrationTool> {
    let mut internal_integrations = vec![
        integration_tool(
            "Date and time tools",
            ToolScope::UserSelectable,
            vec![builtin_tools::time_date::get_time_date_tool()],
        ),
        integration_tool(
            "Web tools",
            ToolScope::UserSelectable,
            vec![builtin_tools::web::get_open_url_tool()],
        ),
        integration_tool(
            "Python tools",
            ToolScope::UserSelectable,
            vec![
                builtin_tools::monty::get_search_tool_functions_definition(),
                builtin_tools::monty::get_tool_definition(),
            ],
        ),
        integration_tool(
            "HTML canvas tools",
            ToolScope::UserSelectable,
            vec![builtin_tools::html_canvas::get_tool_definition()],
        ),
        integration_tool(
            "Bash tools",
            ToolScope::UserSelectable,
            vec![builtin_tools::bashkit::get_tool_definition()],
        ),
        integration_tool(
            "Tools to retrieve documents and read their contents.",
            ToolScope::DocumentIntelligence,
            vec![
                builtin_tools::list_documents::get_tool_definition(),
                builtin_tools::read_document::get_tool_definition(),
                //builtin_tools::read_document_section::get_tool_definition(),
            ],
        ),
    ];

    // Filter by scope if provided
    if let Some(filter_scope) = scope {
        internal_integrations.retain(|integration| integration.scope == filter_scope);
    }

    internal_integrations
}

/// The full list of tools a user can select for the chat.
pub fn get_tools(scope: ToolScope) -> Vec<ToolDefinition> {
    get_integrations(Some(scope))
        .into_iter()
        .flat_map(|integration| integration.definitions)
        .collect()
}

pub async fn get_tools_with_system_openapi(pool: &Pool, scope: ToolScope) -> Vec<ToolDefinition> {
    let mut definitions = get_tools(scope.clone());
    if scope == ToolScope::UserSelectable {
        if let Ok(mut system_defs) = get_system_openapi_tool_definitions(pool).await {
            if !system_defs.is_empty() {
                let system_names: Vec<String> =
                    system_defs.iter().map(|def| def.name.clone()).collect();
                definitions.retain(|def| !system_names.contains(&def.name));
                definitions.append(&mut system_defs);
            }
        }
    }

    definitions
}

/// Returns all built-in user-selectable tool definitions.
pub fn get_chat_tools_user_selected() -> Vec<ToolDefinition> {
    get_tools(ToolScope::UserSelectable)
}

pub async fn get_chat_tools_user_selected_with_system_openapi(pool: &Pool) -> Vec<ToolDefinition> {
    get_tools_with_system_openapi(pool, ToolScope::UserSelectable).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_get_openai_tools_always_returns_builtins() {
        let tools = get_chat_tools_user_selected();
        let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_str()).collect();
        assert!(names.contains(&"get_current_time_and_date"));
        assert!(names.contains(&"open_url"));
        assert!(names.contains(&"search_tool_functions"));
        assert!(names.contains(&"run_python"));
        assert!(names.contains(&"render_html"));
        assert!(names.contains(&"run_bash"));
        assert!(!names.contains(&"list_datasets"));
        assert!(!names.contains(&"list_dataset_files"));
        assert!(!names.contains(&"search_context"));
    }

    #[test]
    fn test_integration_tool_definitions_json() {
        // Get all integrations
        let integrations = get_integrations(None);

        // Verify that there's at least one integration
        assert!(
            !integrations.is_empty(),
            "Expected at least one integration"
        );

        // Check the first integration
        let first_integration = &integrations[0];

        // Verify that definitions_json is not empty
        assert!(
            !first_integration.definitions_json.is_empty(),
            "Expected non-empty definitions_json"
        );

        // Verify that definitions_json is a valid JSON representation of definitions
        let expected_json = serde_json::to_string_pretty(&first_integration.definitions)
            .expect("Failed to serialize definitions to JSON");

        assert_eq!(
            first_integration.definitions_json, expected_json,
            "definitions_json does not match the expected JSON representation"
        );
    }

    #[test]
    fn test_get_integrations_with_scope_filter() {
        // Test filtering by UserSelectable scope
        let user_selectable = get_integrations(Some(ToolScope::UserSelectable));
        assert!(
            !user_selectable.is_empty(),
            "Expected at least one UserSelectable integration"
        );
        for integration in &user_selectable {
            assert_eq!(integration.scope, ToolScope::UserSelectable);
        }

        // Test filtering by DocumentIntelligence scope
        let doc_intelligence = get_integrations(Some(ToolScope::DocumentIntelligence));
        assert!(
            !doc_intelligence.is_empty(),
            "Expected at least one DocumentIntelligence integration"
        );
        for integration in &doc_intelligence {
            assert_eq!(integration.scope, ToolScope::DocumentIntelligence);
        }
    }
}
