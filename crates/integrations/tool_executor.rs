use crate::attachment_as_text::AttachmentAsTextTool;
use crate::attachment_to_markdown::AttachmentToMarkdownTool;
use crate::attachments_list::ListAttachmentsTool;
use crate::external_integration::create_external_integration_tools;
use crate::time_date::TimeDateTool;
use crate::tool::ToolInterface;
use db::Pool;
use openai_api::{ToolCall, ToolCallResult};
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn};

/// Execute a tool call and return a message with the result
pub async fn execute_tool_calls(
    tool_calls: Vec<ToolCall>,
    pool: Option<&Pool>,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Vec<ToolCallResult> {
    info!("Executing {} tool calls", tool_calls.len());

    // Get tool instances with the pool for execution
    debug!("Getting tool instances");
    let tools = get_tools(pool, sub.clone(), conversation_id).await;
    debug!("Got {} tool instances", tools.len());

    let mut tool_results: Vec<ToolCallResult> = Vec::new();
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
    pool: Option<&Pool>,
    sub: Option<String>,
    conversation_id: Option<i64>,
) -> Vec<Arc<dyn ToolInterface>> {
    trace!("Getting available tool instances");

    // Start with internal tools
    let mut tools: Vec<Arc<dyn ToolInterface>> = vec![Arc::new(TimeDateTool)];
    debug!("Added TimeDateTool");

    // Add the attachment tools if a pool is provided
    if let (Some(pool), Some(sub)) = (pool, sub) {
        debug!("Adding attachment tools with database pool");
        tools.push(Arc::new(ListAttachmentsTool::new(
            pool.clone(),
            Some(sub.clone()),
            conversation_id,
        )));
        tools.push(Arc::new(AttachmentAsTextTool::new(
            pool.clone(),
            Some(sub.clone()),
            conversation_id,
        )));
        tools.push(Arc::new(AttachmentToMarkdownTool::new(
            pool.clone(),
            Some(sub.clone()),
            conversation_id,
        )));

        // Get external integration tools
        debug!("Getting external integration tools");
        let external_tools = create_external_integration_tools(pool, sub).await;

        if !external_tools.is_empty() {
            debug!("Found {} external integration tools", external_tools.len());

            // Check for name conflicts and override internal tools
            let mut tool_names = HashSet::new();
            for tool in &tools {
                tool_names.insert(tool.name());
            }

            for external_tool in external_tools {
                let name = external_tool.name();
                if tool_names.contains(&name) {
                    debug!(
                        "External tool {} overrides internal tool with the same name",
                        name
                    );
                    // Remove the internal tool with the same name
                    tools.retain(|t| t.name() != name);
                }
                tools.push(external_tool);
                tool_names.insert(name);
            }

            debug!(
                "Added external integration tools, total tools: {}",
                tools.len()
            );
        }
    } else {
        debug!("Skipping attachment tools and external integrations (no database pool provided)");
    }

    info!("Returning {} tool instances", tools.len());
    tools
}

/// Execute a tool call with a specific set of tools
pub async fn execute_tool_call_with_tools(
    tools: &[Arc<dyn ToolInterface>],
    tool_call: &ToolCall,
) -> ToolCallResult {
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
        let result = tool.execute(&tool_call.function.arguments).await;

        if let Ok(result) = result {
            debug!("Tool execution successful");
            return ToolCallResult {
                id: tool_call.id.clone(),
                name: tool_call.function.name.clone(),
                result,
            };
        } else if let Err(e) = result {
            error!("Tool execution failed: {}", e);
        }
    } else {
        warn!("Tool not found: {}", tool_name);
    }

    debug!("Returning error result for tool call");
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

    #[tokio::test]
    async fn test_get_tools_no_pool() {
        // Test get_tools without a pool
        let tools = get_tools(None, None, None).await;

        // Should only have the TimeDateTool
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name(), "get_current_time_and_date");
    }
}
