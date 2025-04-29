use std::env;
use std::sync::Arc;

// Import the tool trait and weather tool
use crate::tool::ToolInterface;
use crate::weather::WeatherTool;
use openai_api::BionicToolDefinition;

/// Returns a list of available tools
/// Only returns tools if the MCP_ENABLED environment variable is set
pub fn get_tools() -> Vec<Arc<dyn ToolInterface>> {
    vec![Arc::new(WeatherTool)]
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
pub fn get_openai_tools() -> Vec<BionicToolDefinition> {
    tools.iter().map(|tool| tool.get_tool()).collect()
}
