use crate::attachments::AttachmentsTool;
use crate::time_date::TimeDateTool;
use crate::tool::ToolInterface;
use db::Pool;
use openai_api::{ToolCall, ToolCallResult};
use serde_json::json;
use std::sync::Arc;

/// Execute a tool call and return a message with the result
pub async fn execute_tool_calls(
    tool_calls: Vec<ToolCall>,
    pool: Option<&Pool>,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Vec<ToolCallResult> {
    // Get tool instances with the pool for execution
    let tools = get_tools(pool, sub, conversation_id);
    let mut tool_results: Vec<ToolCallResult> = Vec::new();
    for tool_call in tool_calls {
        tool_results.push(execute_tool_call_with_tools(&tools, &tool_call).await);
    }
    tool_results
}

/// Returns a list of available tool instances
/// This requires a pool for tools that need database access
pub fn get_tools(
    pool: Option<&Pool>,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Vec<Arc<dyn ToolInterface>> {
    let mut tools: Vec<Arc<dyn ToolInterface>> = vec![Arc::new(TimeDateTool)];

    // Add the AttachmentsTool if a pool is provided
    if let Some(pool) = pool {
        tools.push(Arc::new(AttachmentsTool::new(
            pool.clone(),
            sub,
            conversation_id,
        )));
    }

    tools
}

/// Execute a tool call with a specific set of tools
pub async fn execute_tool_call_with_tools(
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
        // Execute the tool asynchronously
        let result = tool.execute(&tool_call.function.arguments).await;

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

    #[tokio::test]
    async fn test_execute_tool_call_time_date() {
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

        let result = execute_tool_call_with_tools(&tools, &tool_call).await;
        assert_eq!(result.id, "call_123".to_string());
        assert_eq!(result.name, "get_current_time_and_date".to_string());
    }
}
