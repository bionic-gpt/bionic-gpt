use crate::function_tools;
use openai_api::{Message, ToolCall};

/// Execute a tool call and return a message with the result
pub fn execute_tool_call(tool_call: &ToolCall) -> Result<Message, String> {
    match tool_call.function.name.as_str() {
        "get_weather" => {
            let result = function_tools::execute_weather_function(&tool_call.function.arguments)?;

            Ok(Message {
                role: "tool".to_string(),
                content: result,
                tool_call_id: Some(tool_call.id.clone()),
                name: Some("get_weather".to_string()),
                tool_calls: None,
            })
        }
        _ => Err(format!("Unknown function: {}", tool_call.function.name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openai_api::ToolCallFunction;
    use serde_json::json;

    #[test]
    fn test_execute_tool_call_weather() {
        let tool_call = ToolCall {
            id: "call_123".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "get_weather".to_string(),
                arguments: json!({"location": "San Francisco, CA"}).to_string(),
            },
        };

        let result = execute_tool_call(&tool_call).unwrap();
        assert_eq!(result.role, "tool");
        assert_eq!(result.tool_call_id, Some("call_123".to_string()));
        assert_eq!(result.name, Some("get_weather".to_string()));
    }

    #[test]
    fn test_execute_tool_call_unknown() {
        let tool_call = ToolCall {
            id: "call_456".to_string(),
            r#type: "function".to_string(),
            function: ToolCallFunction {
                name: "unknown_function".to_string(),
                arguments: "{}".to_string(),
            },
        };

        let result = execute_tool_call(&tool_call);
        assert!(result.is_err());
    }
}
