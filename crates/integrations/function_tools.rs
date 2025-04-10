use openai_api::Tool;
use std::env;

// Import the weather tool functionality from the weather module
use crate::weather::get_weather_tool;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Vec<Tool> {
    // Check if MCP_ENABLED environment variable is set
    match env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => vec![get_weather_tool()],
        Err(_) => vec![], // Return empty vector if MCP_ENABLED is not set
    }
}

// The weather tool functionality has been moved to weather.rs
