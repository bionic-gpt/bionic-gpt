//! Example of using function calling with LLMs
//!
//! This simplified example demonstrates how to:
//! 1. Create a conversation with hard-coded function calls and responses
//! 2. Send the complete conversation to an LLM
//! 3. Process the final response

use futures::StreamExt;
use openai_api::{Completion, Message, ToolCall, ToolCallFunction};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::{json, Value};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read environment variables
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url =
        env::var("OPENAI_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

    println!("Function Calling Example (Simplified)");
    println!("------------------------------------");
    println!("Using API URL: {}", api_url);
    println!("Using model: {}", model);
    println!();

    // Create a conversation with hard-coded function calls and responses
    let messages = vec![
        // User question
        Message {
            role: "user".to_string(),
            content: Some("What's the weather like in San Francisco?".to_string()),
            tool_call_id: None,
            tool_calls: None,
            name: None,
        },
        // Assistant response with function call
        Message {
            role: "assistant".to_string(),
            content: None,
            tool_call_id: None,
            tool_calls: Some(vec![ToolCall {
                id: "call_123".to_string(),
                r#type: "function".to_string(),
                function: ToolCallFunction {
                    name: "get_weather".to_string(),
                    arguments: json!({
                        "location": "San Francisco, CA",
                        "unit": "celsius"
                    })
                    .to_string(),
                },
            }]),
            name: None,
        },
        // Tool response
        Message {
            role: "tool".to_string(),
            content: Some(
                json!({
                    "location": "San Francisco, CA",
                    "temperature": 22,
                    "unit": "celsius",
                    "condition": "sunny",
                    "forecast": ["sunny", "partly cloudy", "sunny"]
                })
                .to_string(),
            ),
            tool_call_id: Some("call_123".to_string()),
            name: Some("get_weather".to_string()),
            tool_calls: None,
        },
    ];

    println!("Sending conversation with hard-coded function calls to LLM...");
    println!();

    // Print the conversation for demonstration
    println!("User: {:?}", messages[0].content);
    println!(
        "Assistant: [Function call to get_weather with arguments: {}]",
        messages[1].tool_calls.as_ref().unwrap()[0]
            .function
            .arguments
    );
    println!("Tool response: {:?}", messages[2].content);
    println!();

    // Create a completion request with the complete conversation
    let completion = Completion {
        model,
        stream: Some(true),
        max_tokens: Some(1000),
        temperature: Some(0.7),
        messages,
        tools: None, // No need to define tools since we're not expecting function calls
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

    // Send the request and get the response
    let response = request.send().await?;

    // Check if the response is successful
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("API request failed: {}", error_text).into());
    }

    // Create a stream from the response body
    let mut stream = response.bytes_stream();

    // Process the stream
    println!("Final LLM response:");
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

    Ok(())
}
