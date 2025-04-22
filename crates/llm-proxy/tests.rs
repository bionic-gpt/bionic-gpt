use db::Chat;
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use serde_json::json;
use time::OffsetDateTime;

use crate::{chat_converter::convert_chat_to_messages, prompt::generate_prompt};

#[tokio::test]
async fn test_convert_chat_to_messages_function_calling() {
    // Create a Chat struct with function call data similar to the OpenAI example
    let chat = Chat {
        id: 0,
        conversation_id: 0,
        user_request: "What's the weather like in San Francisco?".to_string(),
        function_call: Some(
            json!({
                "id": "call_123",
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "arguments": {
                        "location": "San Francisco, CA",
                        "unit": "celsius"
                    }
                }
            })
            .to_string(),
        ),
        function_call_results: Some(
            json!({
                "location": "San Francisco, CA",
                "temperature": 22,
                "unit": "celsius",
                "condition": "sunny",
                "forecast": ["sunny", "partly cloudy", "sunny"]
            })
            .to_string(),
        ),
        prompt: "weather query".to_string(),
        prompt_id: 0,
        model_name: "gpt-4".to_string(),
        response: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    };

    // Call convert_chat_to_messages on this struct
    let messages = convert_chat_to_messages(vec![chat]);

    dbg!(&messages);

    // Assert the new expected behavior
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[1].role, ChatCompletionMessageRole::Assistant);
    assert!(messages[1].tool_calls.is_some());
    let tool_calls = messages[1].tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].id, "call_123");
    assert_eq!(tool_calls[0].r#type, "function");
    assert_eq!(tool_calls[0].function.name, "get_weather");

    assert_eq!(messages[2].role, ChatCompletionMessageRole::Tool);
    assert_eq!(messages[2].tool_call_id, Some("call_123".to_string()));
    assert_eq!(messages[2].name, Some("get_weather".to_string()));
}

#[tokio::test]
async fn test_convert_chat_to_messages_function_calling_fallback() {
    // Create a Chat struct with invalid JSON function call data
    let chat = Chat {
        id: 0,
        conversation_id: 0,
        user_request: "What's the weather like in San Francisco?".to_string(),
        function_call: Some("invalid json".to_string()),
        function_call_results: Some("some results".to_string()),
        prompt: "weather query".to_string(),
        prompt_id: 0,
        model_name: "gpt-4".to_string(),
        response: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    };

    // Call convert_chat_to_messages on this struct
    let messages = convert_chat_to_messages(vec![chat]);

    // Assert the fallback behavior
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[1].role, ChatCompletionMessageRole::Function);
    assert_eq!(messages[1].content, Some("invalid json".to_string()));
    assert_eq!(messages[2].role, ChatCompletionMessageRole::Tool);
    assert_eq!(messages[2].content, Some("some results".to_string()));
    assert_eq!(messages[2].tool_call_id, None);
    assert_eq!(messages[2].name, None);
}

#[tokio::test]
async fn test_generate_prompt() {
    let (messages, _chunk_ids) = generate_prompt(
        2048,
        1024,
        1.0,
        Some("You are a helpful asistant".to_string()),
        vec![ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some("How are you today?".to_string()),
            tool_call_id: None,
            tool_calls: None,
            name: None,
            function_call: None,
        }],
        Default::default(),
    )
    .await;

    dbg!(&messages);

    assert!(messages.len() == 2);

    assert!(messages[0].content == Some("You are a helpful asistant".to_string()));
    assert!(messages[1].content == Some("How are you today?".to_string()));
}
