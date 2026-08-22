use crate::builtin_tools;
use crate::types::{ToolCall, ToolResult, ToolResultContent};
use db::Pool;
use rig::tool::ToolDyn;
use rig::OneOrMany;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

/// Execute a tool call and return a message with the result
pub async fn execute_tool_calls(
    tool_calls: Vec<ToolCall>,
    pool: &Pool,
    sub: String,
    conversation_id: i64,
    prompt_id: i32,
) -> Vec<ToolResult> {
    info!("Executing {} tool calls", tool_calls.len());

    // Get tool instances with the pool for execution
    debug!("Getting tool instances");
    let tools = get_tools(pool, sub.clone(), conversation_id, prompt_id).await;
    debug!("Got {} tool instances", tools.len());

    let mut tool_results: Vec<ToolResult> = Vec::new();
    for (i, tool_call) in tool_calls.iter().enumerate() {
        debug!(
            "Executing tool call {}/{}: {}",
            i + 1,
            tool_calls.len(),
            tool_call.function.name
        );
        tool_results.push(execute_tool_call_with_tools(&tools, tool_call).await);
    }

    info!("Completed execution of {} tool calls", tool_calls.len());
    tool_results
}

/// Returns a list of available tool instances
/// This requires a pool for tools that need database access
pub async fn get_tools(
    pool: &Pool,
    sub: String,
    conversation_id: i64,
    prompt_id: i32,
) -> Vec<Arc<dyn ToolDyn>> {
    trace!("Getting available tool instances");

    // Start with internal tools
    let tools: Vec<Arc<dyn ToolDyn>> = vec![Arc::new(builtin_tools::bashkit::BashkitTool::new(
        pool.clone(),
        sub.clone(),
        conversation_id,
        prompt_id,
    ))];

    info!("Returning {} tool instances", tools.len());
    tools
}

/// Execute a tool call with a specific set of tools
pub async fn execute_tool_call_with_tools(
    tools: &[Arc<dyn ToolDyn>],
    tool_call: &ToolCall,
) -> ToolResult {
    let tool_name = &tool_call.function.name;
    info!("Executing tool call: {}", tool_name);
    debug!("Tool call arguments: {}", tool_call.function.arguments);

    // Find the tool with the matching name
    debug!("Searching for tool with name: {}", tool_name);
    let tool = tools
        .iter()
        .find(|t| &t.name() == tool_name)
        .ok_or_else(|| format!("Unknown tool: {}", tool_name));

    if let Ok(tool) = tool {
        debug!("Found matching tool, executing");
        // Execute the tool asynchronously
        let result = tool.call(tool_call.function.arguments.to_string()).await;

        if let Ok(result) = result {
            debug!("Tool execution successful");
            return ToolResult {
                id: tool_call.id.clone(),
                call_id: tool_call.call_id.clone(),
                content: OneOrMany::one(ToolResultContent::text(result)),
            };
        } else if let Err(e) = result {
            error!("Tool execution failed: {}", e);
            return to_error_result(tool_call, json!({"error": e.to_string()}));
        }
    } else {
        warn!("Tool not found: {}", tool_name);
    }

    to_error_result(tool_call, json!({"error": "Problem calling tool"}))
}

fn to_error_result(tool_call: &ToolCall, error: Value) -> ToolResult {
    debug!("Returning error result for tool call");
    ToolResult {
        id: tool_call.id.clone(),
        call_id: tool_call.call_id.clone(),
        content: OneOrMany::one(ToolResultContent::text(error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin_tools::time_date::TimeDateTool;
    use crate::types::{ToolCall, ToolCallFunction};
    use rig::tool::ToolDyn;
    use serde_json::json;

    #[tokio::test]
    async fn test_execute_tool_call_time_date() {
        let time_date_tool: Arc<dyn ToolDyn> = Arc::new(TimeDateTool);
        let tools: Vec<Arc<dyn ToolDyn>> = vec![time_date_tool];

        let tool_call = ToolCall {
            id: "call_123".to_string(),
            call_id: None,
            signature: None,
            additional_params: None,
            function: ToolCallFunction {
                name: "get_current_time_and_date".to_string(),
                arguments: json!({"timezone": "utc"}),
            },
        };

        let result = execute_tool_call_with_tools(&tools, &tool_call).await;
        assert_eq!(result.id, "call_123".to_string());
        let payload = match result.content.first() {
            ToolResultContent::Text(text) => text.text,
            ToolResultContent::Image(_) => String::new(),
        };
        let parsed: Value = serde_json::from_str(&payload).unwrap_or_default();
        assert_eq!(parsed["timezone"], "utc");
    }
}
