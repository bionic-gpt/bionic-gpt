use crate::tool::ToolInterface;
use crate::tools::time_date::TimeDateTool;
use openai_api::ToolCall;
use std::sync::Arc;

// A simple test to verify that our async implementation works correctly
#[tokio::test]
async fn test_async_tool_execution() {
    // Create a TimeDateTool instance
    let time_date_tool = TimeDateTool;

    // Execute the tool
    let result = time_date_tool
        .execute(r#"{"timezone": "utc", "format": "human_readable"}"#)
        .await;

    // Verify the result
    assert!(result.is_ok());
    let result_value = result.unwrap();
    assert!(result_value["current_time"].is_string());
    assert!(result_value["timestamp"].is_number());
    assert!(result_value["timezone"].is_string());
    assert!(result_value["format"].is_string());
}

// Test the execute_tool_call_with_tools function
#[tokio::test]
async fn test_execute_tool_call_with_tools() {
    use crate::tool_executor::execute_tool_call_with_tools;
    use openai_api::ToolCallFunction;
    use serde_json::json;

    // Create a TimeDateTool instance
    let time_date_tool: Arc<dyn ToolInterface> = Arc::new(TimeDateTool);
    let tools: Vec<Arc<dyn ToolInterface>> = vec![time_date_tool];

    // Create a tool call
    let tool_call = ToolCall {
        id: "call_123".to_string(),
        r#type: "function".to_string(),
        function: ToolCallFunction {
            name: "get_current_time_and_date".to_string(),
            arguments: json!({"timezone": "utc"}).to_string(),
        },
    };

    // Execute the tool call
    let result = execute_tool_call_with_tools(&tools, &tool_call).await;

    // Verify the result
    assert_eq!(result.id, "call_123".to_string());
    assert_eq!(result.name, "get_current_time_and_date".to_string());
    assert!(result.result["current_time"].is_string());
}
