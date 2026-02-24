use crate::builtin_tools::time_date::TimeDateTool;
use crate::types::{ToolCall, ToolResultContent};
use rig::tool::ToolDyn;
use serde_json::json;
use std::sync::Arc;

// A simple test to verify that our async implementation works correctly
#[tokio::test]
async fn test_async_tool_execution() {
    // Create a TimeDateTool instance
    let time_date_tool = TimeDateTool;

    // Execute the tool
    let result = time_date_tool
        .call(json!({"timezone": "utc", "format": "human_readable"}).to_string())
        .await;

    // Verify the result
    assert!(result.is_ok());
    let result_value: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
    assert!(result_value["current_time"].is_string());
    assert!(result_value["timestamp"].is_number());
    assert!(result_value["timezone"].is_string());
    assert!(result_value["format"].is_string());
}

// Test the execute_tool_call_with_tools function
#[tokio::test]
async fn test_execute_tool_call_with_tools() {
    use crate::tool_dispatcher::execute_tool_call_with_tools;
    use crate::types::ToolCallFunction;

    // Create a TimeDateTool instance
    let time_date_tool: Arc<dyn ToolDyn> = Arc::new(TimeDateTool);
    let tools: Vec<Arc<dyn ToolDyn>> = vec![time_date_tool];

    // Create a tool call
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        call_id: None,
        function: ToolCallFunction {
            name: "get_current_time_and_date".to_string(),
            arguments: json!({"timezone": "utc"}),
        },
    };

    // Execute the tool call
    let result = execute_tool_call_with_tools(&tools, &tool_call).await;

    // Verify the result
    assert_eq!(result.id, "call_123".to_string());
    let payload = match result.content.first() {
        ToolResultContent::Text(text) => text.text,
        ToolResultContent::Image(_) => String::new(),
    };
    let parsed: serde_json::Value = serde_json::from_str(&payload).unwrap_or_default();
    assert!(parsed["current_time"].is_string());
}
