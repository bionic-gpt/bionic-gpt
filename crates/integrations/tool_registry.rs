// Import the tool trait and time date tool
use crate::tools;
use openai_api::BionicToolDefinition;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum ToolScope {
    UserSelectable,
    DocumentIntelligence,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct IntegrationTool {
    pub title: String,
    pub scope: ToolScope,
    pub definitions: Vec<BionicToolDefinition>,
    pub definitions_json: String,
}

pub fn get_integrations(scope: Option<ToolScope>) -> Vec<IntegrationTool> {
    let mut internal_integrations = vec![
        IntegrationTool {
            scope: ToolScope::UserSelectable,
            title: "Date and time tools".into(),
            definitions: vec![tools::time_date::get_time_date_tool()],
            definitions_json: serde_json::to_string_pretty(&vec![
                tools::time_date::get_time_date_tool(),
            ])
            .expect("Failed to serialize time_date_tool to JSON"),
        },
        IntegrationTool {
            scope: ToolScope::UserSelectable,
            title: "Web tools".into(),
            definitions: vec![tools::web::get_open_url_tool()],
            definitions_json: serde_json::to_string_pretty(&vec![tools::web::get_open_url_tool()])
                .expect("Failed to serialize web tools to JSON"),
        },
        IntegrationTool {
            scope: ToolScope::DocumentIntelligence,
            title: "Tools to retrieve documents and read their contents.".into(),
            definitions: vec![
                tools::list_documents::get_tool_definition(),
                tools::read_document_section::get_tool_definition(),
            ],
            definitions_json: serde_json::to_string_pretty(&vec![
                tools::list_documents::get_tool_definition(),
                tools::read_document_section::get_tool_definition(),
            ])
            .expect("Failed to serialize attachment tools to JSON"),
        },
    ];

    // Filter by scope if provided
    if let Some(filter_scope) = scope {
        internal_integrations.retain(|integration| integration.scope == filter_scope);
    }

    internal_integrations
}

/// The full list of tools a user can select for the chat.
pub fn get_tools(scope: ToolScope) -> Vec<BionicToolDefinition> {
    get_integrations(Some(scope))
        .into_iter()
        .flat_map(|integration| integration.definitions)
        .collect()
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
///
/// If enabled_tools is provided, only returns tools with names in that list
pub fn get_chat_tools_user_selected(
    enabled_tools: Option<&Vec<String>>,
) -> Vec<BionicToolDefinition> {
    let all_tool_definitions = get_tools(ToolScope::UserSelectable);

    match enabled_tools {
        Some(tool_names) if !tool_names.is_empty() => all_tool_definitions
            .into_iter()
            .filter(|tool_def| tool_names.contains(&tool_def.function.name))
            .collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::time_date::get_time_date_tool;
    use serde_json;

    #[test]
    fn test_get_openai_tools_none() {
        // When enabled_tools is None, it should return no tools
        let tools = get_chat_tools_user_selected(None);
        assert!(
            tools.is_empty(),
            "Expected empty tools list when enabled_tools is None"
        );
    }

    #[test]
    fn test_get_openai_tools_empty() {
        // When enabled_tools is Some but empty, it should return no tools
        let empty_vec = vec![];
        let tools = get_chat_tools_user_selected(Some(&empty_vec));
        assert!(
            tools.is_empty(),
            "Expected empty tools list when enabled_tools is empty"
        );
    }

    #[test]
    fn test_get_openai_tools_with_valid_names() {
        // Override the get_tools function for this test
        let time_date_tool = get_time_date_tool();
        let time_date_tool_name = time_date_tool.function.name.clone();

        // When enabled_tools contains valid tool names, it should return only those tools
        let valid_names = vec![time_date_tool_name];
        let tools = get_chat_tools_user_selected(Some(&valid_names));

        assert_eq!(tools.len(), 1, "Expected exactly one tool");
        assert_eq!(tools[0].function.name, "get_current_time_and_date");
    }

    #[test]
    fn test_get_openai_tools_with_invalid_names() {
        // When enabled_tools contains non-existent tool names, it should return no tools
        let invalid_names = vec!["non_existent_tool".to_string()];
        let tools = get_chat_tools_user_selected(Some(&invalid_names));
        assert!(
            tools.is_empty(),
            "Expected empty tools list for non-existent tool names"
        );
    }

    #[test]
    fn test_get_openai_tools_with_mixed_names() {
        // Get the actual tool name from the implementation
        let time_date_tool = get_time_date_tool();
        let time_date_tool_name = time_date_tool.function.name.clone();

        // When enabled_tools contains both valid and invalid tool names,
        // it should return only the valid ones
        let mixed_names = vec![time_date_tool_name, "non_existent_tool".to_string()];
        let tools = get_chat_tools_user_selected(Some(&mixed_names));

        assert_eq!(tools.len(), 1, "Expected exactly one tool");
        assert_eq!(tools[0].function.name, "get_current_time_and_date");
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
