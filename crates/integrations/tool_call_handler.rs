use crate::function_executor::execute_tool_call;
use openai_api::{CompletionResponse, ToolCall, ToolCallFunction};

/// Handles a tool call event by extracting the JSON data, executing the function if it's a tool call,
/// and returning the function call and result.
///
/// Returns a tuple of (Option<String>, Option<String>) where:
/// - The first element is the function call JSON (if present)
/// - The second element is the function result JSON (if present)
pub fn handle_tool_call(delta: String) -> (Option<String>, Option<String>) {
    // Try to parse the delta as a CompletionResponse
    match serde_json::from_str::<CompletionResponse>(&delta) {
        Ok(response) => {
            // Check if there are any tool calls
            if let Some(choice) = response.choices.first() {
                if let Some(tool_call_json) = choice.delta.tool_calls.first() {
                    // Extract the function call JSON
                    let function_call = serde_json::to_string(tool_call_json).unwrap_or_default();

                    // Convert from the JSON struct to our domain struct
                    let tool_call = ToolCall {
                        id: tool_call_json.id.clone(),
                        r#type: tool_call_json.call_type.clone(),
                        function: ToolCallFunction {
                            name: tool_call_json.function.name.clone(),
                            arguments: tool_call_json.function.arguments.clone(),
                        },
                    };

                    // Log the structured data
                    tracing::info!(
                        tool_call_id = %tool_call.id,
                        function_name = %tool_call.function.name,
                        arguments = %tool_call.function.arguments,
                        "Executing tool call"
                    );

                    // Execute the tool call
                    match execute_tool_call(&tool_call) {
                        Ok(result) => {
                            // Return the function call and result
                            match serde_json::to_string(&result) {
                                Ok(json) => return (Some(function_call), Some(json)),
                                Err(e) => {
                                    return (
                                        Some(function_call),
                                        Some(format!(
                                            "{{\"error\": \"Failed to serialize result: {}\"}}",
                                            e
                                        )),
                                    )
                                }
                            }
                        }
                        Err(err) => {
                            // Return the function call and error
                            tracing::error!("Failed to execute tool call: {}", err);
                            return (
                                Some(function_call),
                                Some(format!("{{\"error\": \"{}\"}}", err)),
                            );
                        }
                    }
                }
            }
        }
        Err(e) => {
            // Not a valid CompletionResponse, might be regular text
            tracing::debug!("Not a tool call JSON: {:?}", e);
        }
    }

    // If we get here, it wasn't a tool call or parsing failed
    (None, None)
}
