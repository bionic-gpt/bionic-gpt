// Import the tool trait and time date tool
use crate::attachment_as_text::get_attachment_as_text_tool;
use crate::attachment_to_markdown::get_attachment_to_markdown_tool;
use crate::attachments_list::get_list_attachments_tool;
use crate::open_api_v3::open_api_to_definition_legacy;
use crate::time_date::get_time_date_tool;
use db::Integration;
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

pub fn get_integrations(external_integrations: Vec<Integration>) -> Vec<IntegrationTool> {
    let mut internal_integrations = vec![
        IntegrationTool {
            scope: ToolScope::UserSelectable,
            title: "Date and time tools".into(),
            definitions: vec![get_time_date_tool()],
            definitions_json: serde_json::to_string_pretty(&vec![get_time_date_tool()])
                .expect("Failed to serialize time_date_tool to JSON"),
        },
        IntegrationTool {
            scope: ToolScope::DocumentIntelligence,
            title: "Tools to retrieve documents and read their contents.".into(),
            definitions: vec![
                get_list_attachments_tool(),
                get_attachment_to_markdown_tool(),
                get_attachment_as_text_tool(),
            ],
            definitions_json: serde_json::to_string_pretty(&vec![
                get_list_attachments_tool(),
                get_attachment_to_markdown_tool(),
                get_attachment_as_text_tool(),
            ])
            .expect("Failed to serialize attachment tools to JSON"),
        },
    ];

    let mut external_integrations = get_external_integrations(external_integrations);

    internal_integrations.append(&mut external_integrations);

    internal_integrations
}

// Convert integrations from the DB into IntegrationTool
fn get_external_integrations(integrations: Vec<Integration>) -> Vec<IntegrationTool> {
    integrations
        .iter()
        .filter_map(|integration| {
            if let Some(definition) = &integration.definition {
                let oas3 = oas3::from_json(definition.to_string());
                if let Ok(oas3) = oas3 {
                    let openai_definitions = open_api_to_definition_legacy(oas3.clone());
                    let definitions_json = serde_json::to_string_pretty(&openai_definitions);
                    if let Ok(definitions_json) = definitions_json {
                        Some(IntegrationTool {
                            scope: ToolScope::UserSelectable,
                            title: integration.name.clone(),
                            definitions: openai_definitions.clone(),
                            definitions_json,
                        })
                    } else {
                        tracing::error!("Failed to convert definitions to JSON");
                        None
                    }
                } else {
                    tracing::error!(
                        "Failed to convert JSON in DB to oas3 for integration {}",
                        integration.id
                    );
                    None
                }
            } else {
                tracing::error!("This integration doesn't have a definition");
                None
            }
        })
        .collect()
}

pub fn get_tools_for_attachments() -> Vec<BionicToolDefinition> {
    vec![
        get_list_attachments_tool(),
        get_attachment_to_markdown_tool(),
        get_attachment_as_text_tool(),
    ]
}

/// The name and descriptions of the tools the user can select from
pub fn get_user_selectable_tools_for_chat_ui() -> Vec<(String, String)> {
    get_user_selectable_tools_for_chat()
        .iter()
        .map(|tool| {
            let tool_def = tool.function.description.clone().unwrap_or("".to_string());
            let tool_id = tool.function.name.clone();

            // Use the tool ID as the display name
            // This keeps the display name in one place only
            (tool_id, tool_def)
        })
        .collect()
}

/// The full list of tools a user can select for the chat.
fn get_user_selectable_tools_for_chat() -> Vec<BionicToolDefinition> {
    vec![get_time_date_tool()]
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
///
/// If enabled_tools is provided, only returns tools with names in that list
pub fn get_chat_tools_user_selected(
    enabled_tools: Option<&Vec<String>>,
) -> Vec<BionicToolDefinition> {
    let all_tool_definitions = get_user_selectable_tools_for_chat();

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
    use crate::time_date::get_time_date_tool;
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
        let integrations = get_integrations(vec![]);

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
}
