use std::sync::Arc;

// Import the tool trait and time date tool
use crate::time_date::TimeDateTool;
use crate::tool::ToolInterface;
use openai_api::BionicToolDefinition;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Option<Vec<Arc<dyn ToolInterface>>> {
    Some(vec![Arc::new(TimeDateTool)])
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
pub fn get_openai_tools() -> Option<Vec<BionicToolDefinition>> {
    if let Some(tools) = get_tools() {
        return Some(tools.iter().map(|tool| tool.get_tool()).collect());
    }
    None
}
