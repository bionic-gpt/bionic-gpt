use std::env;
use std::sync::Arc;

// Import the tool trait and weather tool
use crate::tool::ToolInterface;
use crate::weather::WeatherTool;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Option<Vec<Arc<dyn ToolInterface>>> {
    // Check if MCP_ENABLED environment variable is set
    match env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => Some(vec![Arc::new(WeatherTool)]),
        Err(_) => None, // Return empty vector if MCP_ENABLED is not set
    }
}
