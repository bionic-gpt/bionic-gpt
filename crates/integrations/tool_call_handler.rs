use axum::response::sse::Event;
use openai_api::{CompletionResponse, ToolCall, ToolCallFunction};

/// Handles a tool call event by extracting the JSON data into structured types
/// and logging the information using tracing.
pub fn handle_tool_call(delta: String) -> Result<Event, axum::Error> {
    // Parse the JSON from delta using serde
    match serde_json::from_str::<CompletionResponse>(&delta) {
        Ok(response) => {
            // Process each tool call in the response
            for choice in &response.choices {
                for tool_call_json in &choice.delta.tool_calls {
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
                        "Extracted tool call"
                    );
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to parse tool call JSON: {:?}", e);
            // Continue even if parsing fails, to maintain the stream
        }
    }

    // Return the original event
    Ok(Event::default().data(delta))
}
