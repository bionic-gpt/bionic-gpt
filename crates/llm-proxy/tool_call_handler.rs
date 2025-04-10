use super::sse_chat_enricher::CompletionChunk;
use crate::{ToolCall, ToolCallFunction};
use axum::response::sse::Event;
use serde::Deserialize;

/// Structs for deserializing the tool call JSON response
#[derive(Debug, Deserialize)]
struct ToolCallJson {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: ToolCallFunctionJson,
}

#[derive(Debug, Deserialize)]
struct ToolCallFunctionJson {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct DeltaJson {
    #[serde(default)]
    tool_calls: Vec<ToolCallJson>,
}

#[derive(Debug, Deserialize)]
struct ChoiceJson {
    delta: DeltaJson,
}

#[derive(Debug, Deserialize)]
struct CompletionResponse {
    choices: Vec<ChoiceJson>,
}

/// Handles a tool call event by extracting the JSON data into structured types
/// and logging the information using tracing.
pub fn handle_tool_call(completion_chunk: CompletionChunk) -> Result<Event, axum::Error> {
    // Parse the JSON from delta using serde
    match serde_json::from_str::<CompletionResponse>(&completion_chunk.delta) {
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
    Ok(Event::default().data(completion_chunk.delta))
}
