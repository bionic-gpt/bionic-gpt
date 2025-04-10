use openai_api::Tool;
use std::env;
use std::sync::Arc;

// Import the tool trait and weather tool
use crate::tool::ToolInterface;
use crate::weather::WeatherTool;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Vec<Arc<dyn ToolInterface>> {
    // Check if MCP_ENABLED environment variable is set
    match env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => vec![Arc::new(WeatherTool)],
        Err(_) => vec![], // Return empty vector if MCP_ENABLED is not set
    }
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
pub fn get_openai_tools() -> Vec<Tool> {
    get_tools().iter().map(|tool| tool.get_tool()).collect()
}
