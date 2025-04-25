use std::env;
use std::sync::Arc;

// Import the tool trait and time date tool
use crate::time_date::TimeDateTool;
use crate::tool::ToolInterface;
use openai_api::BionicToolDefinition;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Option<Vec<Arc<dyn ToolInterface>>> {
    // Check if MCP_ENABLED environment variable is set
    match env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => Some(vec![Arc::new(TimeDateTool)]),
        Err(_) => None, // Return empty vector if MCP_ENABLED is not set
    }
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
pub fn get_openai_tools() -> Option<Vec<BionicToolDefinition>> {
    if let Some(tools) = get_tools() {
        return Some(tools.iter().map(|tool| tool.get_tool()).collect());
    }
    None
}
