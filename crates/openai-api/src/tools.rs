//! Tool implementations for OpenAI API
//!
//! This module provides a simple implementation of tools that can be used with the OpenAI API.

use crate::{Message, Tool, ToolCall};
use serde_json::{json, Value};
use std::sync::Arc;

/// Tool interface trait that defines the common functionality for all tools
pub trait ToolInterface: Send + Sync {
    /// Returns the tool definition
    fn get_tool(&self) -> Tool;

    /// Executes the tool with the given arguments
    fn execute(&self, arguments: &str) -> Result<String, String>;

    /// Returns the name of the tool
    fn name(&self) -> String {
        self.get_tool().function.name.clone()
    }
}

/// A tool that provides weather information
pub struct WeatherTool;

impl ToolInterface for WeatherTool {
    fn get_tool(&self) -> Tool {
        get_weather_tool()
    }

    fn execute(&self, arguments: &str) -> Result<String, String> {
        execute_weather_function(arguments)
    }
}

/// Returns a Tool definition for the weather function
pub fn get_weather_tool() -> Tool {
    Tool {
        r#type: "function".to_string(),
        function: crate::FunctionDefinition {
            name: "get_weather".to_string(),
            description: "Get the current weather in a given location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "The temperature unit to use"
                    }
                },
                "required": ["location"]
            }),
        },
    }
}

/// Execute the weather function with the given arguments
/// This is a mock implementation that returns fixed data
pub fn execute_weather_function(arguments: &str) -> Result<String, String> {
    let args: Value =
        serde_json::from_str(arguments).map_err(|e| format!("Failed to parse arguments: {}", e))?;

    let location = args["location"]
        .as_str()
        .ok_or_else(|| "Location is required".to_string())?;

    let unit = args["unit"].as_str().unwrap_or("celsius");

    // Mock implementation - return fixed data
    let temp = if unit == "celsius" { 22 } else { 72 };
    let condition = "sunny";

    Ok(json!({
        "location": location,
        "temperature": temp,
        "unit": unit,
        "condition": condition,
        "forecast": ["sunny", "partly cloudy", "sunny"]
    })
    .to_string())
}

/// Returns a list of available tools
pub fn get_tools() -> Vec<Arc<dyn ToolInterface>> {
    // Check if DANGER_JWT_OVERRIDE environment variable is set
    match std::env::var("DANGER_JWT_OVERRIDE") {
        Ok(_) => vec![Arc::new(WeatherTool)],
        Err(_) => vec![], // Return empty vector if DANGER_JWT_OVERRIDE is not set
    }
}

/// Returns a list of available OpenAI tool definitions
pub fn get_openai_tools() -> Vec<Tool> {
    get_tools().iter().map(|tool| tool.get_tool()).collect()
}

/// Execute a tool call and return a message with the result
pub fn execute_tool_call(tool_call: &ToolCall) -> Result<Message, String> {
    // Get all available tools
    let tools = get_tools();
    execute_tool_call_with_tools(&tools, tool_call)
}

/// Execute a tool call with a specific set of tools
pub fn execute_tool_call_with_tools(
    tools: &[Arc<dyn ToolInterface>],
    tool_call: &ToolCall,
) -> Result<Message, String> {
    let function_name = &tool_call.function.name;

    // Find the tool with the matching name
    let tool = tools
        .iter()
        .find(|t| &t.name() == function_name)
        .ok_or_else(|| format!("Unknown function: {}", function_name))?;

    // Execute the tool
    let result = tool.execute(&tool_call.function.arguments)?;

    Ok(Message {
        role: "tool".to_string(),
        content: result,
        tool_call_id: Some(tool_call.id.clone()),
        name: Some(function_name.clone()),
        tool_calls: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToolCallFunction;
    use serde_json::json;

    #[test]
    fn test_get_weather_tool() {
        let tool = get_weather_tool();
        assert_eq!(tool.r#type, "function");
        assert_eq!(tool.function.name, "get_weather");
    }

    #[test]
    fn test_execute_weather_function_valid() {
        let args = r#"{"location": "San Francisco, CA", "unit": "celsius"}"#;
        let result = execute_weather_function(args).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["location"], "San Francisco, CA");
        assert_eq!(parsed["temperature"], 22);
        assert_eq!(parsed["unit"], "celsius");
    }

    #[test]
    fn test_execute_weather_function_missing_location() {
        let args = r#"{"unit": "celsius"}"#;
        let result = execute_weather_function(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_tool_call() {
        let weather_tool: Arc<dyn ToolInterface> = Arc::new(WeatherTool);
        let tools: Vec<Arc<dyn ToolInterface>> = vec![weather_tool];

        let tool_call = ToolCall {
            id: "call_123".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "get_weather".to_string(),
                arguments: json!({"location": "San Francisco, CA"}).to_string(),
            },
        };

        let result = execute_tool_call_with_tools(&tools, &tool_call).unwrap();
        assert_eq!(result.role, "tool");
        assert_eq!(result.tool_call_id, Some("call_123".to_string()));
        assert_eq!(result.name, Some("get_weather".to_string()));
    }
}
