use crate::tool::ToolInterface;
use crate::tool_registry;
use openai_api::{ToolCall, ToolCallResult};
use serde_json::json;
use std::sync::Arc;

/// Execute a tool call and return a message with the result
pub fn execute_tool_calls(tool_calls: Vec<ToolCall>) -> Vec<ToolCallResult> {
    let tools = tool_registry::get_tools();
    let mut tool_results: Vec<ToolCallResult> = Vec::new();
    if let Some(tools) = tools {
        for tool_call in tool_calls {
            tool_results.push(execute_tool_call_with_tools(&tools, &tool_call));
        }
    }
    tool_results
}

/// Execute a tool call with a specific set of tools
pub fn execute_tool_call_with_tools(
    tools: &[Arc<dyn ToolInterface>],
    tool_call: &ToolCall,
) -> ToolCallResult {
    let tool_name = &tool_call.function.name;

    // Find the tool with the matching name
    let tool = tools
        .iter()
        .find(|t| &t.name() == tool_name)
        .ok_or_else(|| format!("Unknown tool: {}", tool_name));

    if let Ok(tool) = tool {
        // Execute the tool
        let result = tool.execute(&tool_call.function.arguments);

        if let Ok(result) = result {
            return ToolCallResult {
                id: tool_call.id.clone(),
                name: tool_call.function.name.clone(),
                result,
            };
        }
    }

    ToolCallResult {
        id: tool_call.id.clone(),
        name: tool_call.function.name.clone(),
        result: json!({"error": "Problem calling tool"}).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time_date::TimeDateTool;
    use openai_api::{ToolCall, ToolCallFunction};
    use serde_json::json;

    #[test]
    fn test_execute_tool_call_time_date() {
        let time_date_tool: Arc<dyn ToolInterface> = Arc::new(TimeDateTool);
        let tools: Vec<Arc<dyn ToolInterface>> = vec![time_date_tool];

        let tool_call = ToolCall {
            id: "call_123".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "get_current_time_and_date".to_string(),
                arguments: json!({"timezone": "utc"}).to_string(),
            },
        };

        let result = execute_tool_call_with_tools(&tools, &tool_call);
        assert_eq!(result.id, "call_123".to_string());
        assert_eq!(result.name, "get_current_time_and_date".to_string());
    }
}
