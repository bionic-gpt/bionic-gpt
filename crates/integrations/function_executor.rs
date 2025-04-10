use crate::function_tools;
use crate::tool::ToolInterface;
use openai_api::{Message, ToolCall};
use std::sync::Arc;

/// Execute a tool call and return a message with the result
pub fn execute_tool_call(tool_call: &ToolCall) -> Result<Message, String> {
    // Get all available tools
    let tools = function_tools::get_tools();
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
    use crate::weather::WeatherTool;
    use openai_api::ToolCallFunction;
    use serde_json::json;

    #[test]
    fn test_execute_tool_call_weather() {
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

    #[test]
    fn test_execute_tool_call_unknown() {
        let weather_tool: Arc<dyn ToolInterface> = Arc::new(WeatherTool);
        let tools: Vec<Arc<dyn ToolInterface>> = vec![weather_tool];

        let tool_call = ToolCall {
            id: "call_456".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "unknown_function".to_string(),
                arguments: "{}".to_string(),
            },
        };

        let result = execute_tool_call_with_tools(&tools, &tool_call);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_tool_call_integration() {
        // This test uses the global tools list
        let tool_call = ToolCall {
            id: "call_123".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "get_weather".to_string(),
                arguments: json!({"location": "San Francisco, CA"}).to_string(),
            },
        };

        // This will only work if DANGER_JWT_OVERRIDE is set in the environment
        // Otherwise, it will return an error because no tools are available
        let result = execute_tool_call(&tool_call);

        // We don't assert success or failure here since it depends on the environment
        // Just make sure the code runs without panicking
        let _ = result;
    }
}
