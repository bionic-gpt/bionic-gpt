// Import the tool trait and time date tool
use crate::builtin_tools;
use crate::types::ToolDefinition;
/// Returns the fixed model-facing tool definitions.
pub fn get_chat_tool_definitions() -> Vec<ToolDefinition> {
    vec![builtin_tools::bashkit::get_tool_definition()]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chat_tools_are_fixed_builtins() {
        let tools = get_chat_tool_definitions();
        let names: Vec<&str> = tools.iter().map(|tool| tool.name.as_str()).collect();
        assert!(names.contains(&"run_bash"));
        assert!(!names.contains(&"open_url"));
        assert!(!names.contains(&"run_python"));
        assert!(!names.contains(&"get_current_time_and_date"));
        assert!(!names.contains(&"search_tool_functions"));
        assert!(!names.contains(&"render_html"));
        assert!(!names.contains(&"list_datasets"));
        assert!(!names.contains(&"list_dataset_files"));
        assert!(!names.contains(&"search_context"));
    }
}
