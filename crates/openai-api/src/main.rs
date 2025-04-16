//! Example of using function calling with LLMs
//!
//! This example demonstrates how to:
//! 1. Make a request to an LLM with a tool definition
//! 2. Handle the streaming response
//! 3. Detect tool calls in the response
//! 4. Execute the tool
//! 5. Send the tool result back to the LLM

use futures::StreamExt;
use openai_api::{execute_tool_call, get_openai_tools};
use openai_api::{Completion, Message, ToolCall};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::Value;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read environment variables
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url =
        env::var("OPENAI_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

    println!("Function Calling Example");
    println!("------------------------");
    println!("Using API URL: {}", api_url);
    println!("Using model: {}", model);
    println!();

    // Create a user message that should trigger the weather tool
    let user_message = Message {
        role: "user".to_string(),
        content: "What's the weather like in San Francisco?".to_string(),
        tool_call_id: None,
        tool_calls: None,
        name: None,
    };

    println!("Sending user message: {}", user_message.content);
    println!();

    // Create a completion request with the weather tool
    let completion = Completion {
        model,
        stream: Some(true),
        max_tokens: Some(1000),
        temperature: Some(0.7),
        messages: vec![user_message],
        tools: Some(get_openai_tools()),
        tool_choice: None,
    };

    // Convert the completion to JSON
    let completion_json = serde_json::to_string(&completion)?;

    // Create a reqwest client
    let client = Client::new();

    // Build the request
    let request = client
        .post(format!("{}/chat/completions", api_url))
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(completion_json);

    println!("Sending request to LLM...");

    // Send the request and get the response
    let response = request.send().await?;

    // Check if the response is successful
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("API request failed: {}", error_text).into());
    }

    println!("Received response, processing stream...");
    println!();

    // Create a stream from the response body
    let mut stream = response.bytes_stream();

    // Variables to store the accumulated response
    let mut accumulated_text = String::new();
    let mut tool_call_detected = false;
    let mut tool_call: Option<ToolCall> = None;

    // Process the stream
    while let Some(item) = stream.next().await {
        let chunk = item?;
        let chunk_str = String::from_utf8_lossy(&chunk);

        // Split the chunk by lines and process each line
        for line in chunk_str.lines() {
            // Skip empty lines and "data: [DONE]" messages
            if line.is_empty() || line == "data: [DONE]" {
                continue;
            }

            // Check if the line starts with "data: "
            if let Some(data) = line.strip_prefix("data: ") {
                // Try to parse the data as a JSON object
                if let Ok(json) = serde_json::from_str::<Value>(data) {
                    // Process the JSON data
                    if let Some(choices) = json.get("choices") {
                        if let Some(choice) = choices.get(0) {
                            if let Some(delta) = choice.get("delta") {
                                // Check for tool calls
                                if let Some(tool_calls) = delta.get("tool_calls") {
                                    if !tool_calls.is_array()
                                        || tool_calls.as_array().unwrap().is_empty()
                                    {
                                        continue;
                                    }

                                    // Extract the tool call
                                    let tool_call_json = tool_calls.get(0).unwrap();

                                    // If this is the first part of a tool call
                                    if !tool_call_detected {
                                        tool_call_detected = true;
                                        println!("Tool call detected!");

                                        // Initialize the tool call
                                        tool_call = Some(ToolCall {
                                            id: tool_call_json
                                                .get("id")
                                                .unwrap()
                                                .as_str()
                                                .unwrap()
                                                .to_string(),
                                            r#type: "function".to_string(),
                                            function: openai_api::ToolCallFunction {
                                                name: tool_call_json
                                                    .get("function")
                                                    .unwrap()
                                                    .get("name")
                                                    .unwrap()
                                                    .as_str()
                                                    .unwrap()
                                                    .to_string(),
                                                arguments: "".to_string(),
                                            },
                                        });
                                    }

                                    // Update the tool call arguments if present
                                    if let Some(function) = tool_call_json.get("function") {
                                        if let Some(arguments) = function.get("arguments") {
                                            if let Some(ref mut tc) = tool_call {
                                                tc.function.arguments +=
                                                    arguments.as_str().unwrap_or("");
                                            }
                                        }
                                    }
                                } else if let Some(content) = delta.get("content") {
                                    // Regular content
                                    if let Some(content_str) = content.as_str() {
                                        print!("{}", content_str);
                                        accumulated_text.push_str(content_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // If a tool call was detected, execute it and send the result back
    if let Some(tc) = tool_call {
        println!("\n\nExecuting tool call: {}", tc.function.name);
        println!("Arguments: {}", tc.function.arguments);

        // Execute the tool call
        match execute_tool_call(&tc) {
            Ok(result) => {
                println!("\nTool result: {}", result.content);

                // Create a new completion request with the tool result
                let mut messages = completion.messages.clone();
                messages.push(Message {
                    role: "assistant".to_string(),
                    content: "".to_string(),
                    tool_call_id: None,
                    tool_calls: Some(vec![tc.clone()]),
                    name: None,
                });
                messages.push(result);

                // Create a new completion
                let new_completion = Completion {
                    model: completion.model,
                    stream: Some(true),
                    max_tokens: completion.max_tokens,
                    temperature: completion.temperature,
                    messages,
                    tools: completion.tools,
                    tool_choice: None,
                };

                // Convert the new completion to JSON
                let new_completion_json = serde_json::to_string(&new_completion)?;

                println!("\nSending tool result back to LLM...");

                // Send the new request
                let new_request = client
                    .post(format!("{}/chat/completions", api_url))
                    .header(AUTHORIZATION, format!("Bearer {}", api_key))
                    .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                    .body(new_completion_json);

                // Get the new response
                let new_response = new_request.send().await?;

                // Check if the new response is successful
                if !new_response.status().is_success() {
                    let error_text = new_response.text().await?;
                    return Err(format!("API request failed: {}", error_text).into());
                }

                // Create a stream from the new response body
                let mut new_stream = new_response.bytes_stream();

                // Process the new stream
                println!("\nFinal response:");
                while let Some(item) = new_stream.next().await {
                    let chunk = item?;
                    let chunk_str = String::from_utf8_lossy(&chunk);

                    // Split the chunk by lines and process each line
                    for line in chunk_str.lines() {
                        // Skip empty lines and "data: [DONE]" messages
                        if line.is_empty() || line == "data: [DONE]" {
                            continue;
                        }

                        // Check if the line starts with "data: "
                        if let Some(data) = line.strip_prefix("data: ") {
                            // Try to parse the data as a JSON object
                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                // Process the JSON data
                                if let Some(choices) = json.get("choices") {
                                    if let Some(choice) = choices.get(0) {
                                        if let Some(delta) = choice.get("delta") {
                                            if let Some(content) = delta.get("content") {
                                                // Regular content
                                                if let Some(content_str) = content.as_str() {
                                                    print!("{}", content_str);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                println!();
            }
            Err(err) => {
                println!("Error executing tool call: {}", err);
            }
        }
    } else {
        println!("\n\nNo tool calls detected in the response.");
    }

    Ok(())
}
