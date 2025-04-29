use std::sync::Arc;

// Import the tool trait and time date tool
use crate::time_date::TimeDateTool;
use crate::tool::ToolInterface;
use openai_api::BionicToolDefinition;

/// Returns a list of available tools
pub fn get_tools() -> Vec<Arc<dyn ToolInterface>> {
    vec![Arc::new(TimeDateTool)]
}

/// Returns a list of available OpenAI tool definitions
/// This is for backward compatibility
pub fn get_openai_tools() -> Vec<BionicToolDefinition> {
    get_tools().iter().map(|tool| tool.get_tool()).collect()
}
