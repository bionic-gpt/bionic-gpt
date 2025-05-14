// Import the tool trait and time date tool
use crate::attachments_list::get_list_attachments_tool;
use crate::attachments_read::get_read_attachment_tool;
use crate::time_date::get_time_date_tool;
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
}

pub fn get_all_integrations() -> Vec<IntegrationTool> {
    vec![
        IntegrationTool {
            scope: ToolScope::UserSelectable,
            title: "Date and time tools".into(),
            definitions: vec![get_time_date_tool()],
        },
        IntegrationTool {
            scope: ToolScope::DocumentIntelligence,
            title: "Tools to retrieve documents and read their contents.".into(),
            definitions: vec![get_list_attachments_tool(), get_read_attachment_tool()],
        },
    ]
}

pub fn get_tools_for_attachments() -> Vec<BionicToolDefinition> {
    vec![get_list_attachments_tool(), get_read_attachment_tool()]
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
}
