use db::Chat;
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};
use serde_json::json;
use time::OffsetDateTime;

use crate::{chat_converter::convert_chat_to_messages, prompt::generate_prompt};

#[tokio::test]
async fn test_convert_chat_to_messages_tool_calling() {
    // Create a Chat struct with function call data similar to the OpenAI example
    let chat = Chat {
        id: 0,
        conversation_id: 0,
        user_request: "What's the current time in San Francisco?".to_string(),
        tool_calls: Some(
            json!([{
                "id": "call_123",
                "type": "function",
                "function": {
                    "name": "get_current_time_and_date",
                    "arguments": json!({
                        "timezone": "local",
                        "format": "human_readable"
                    }).to_string()
                }
            }])
            .to_string(),
        ),
        tool_call_results: Some(
            json!([{
                "id": "call_123",
                "name": "get_current_time_and_date",
                "result": json!({
                    "current_time": "2025-04-25 09:25:00 PDT",
                    "timestamp": 1745123100,
                    "timezone": "local",
                    "format": "human_readable"
                }).to_string()
            }])
            .to_string(),
        ),
        prompt: "time query".to_string(),
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
    assert_eq!(tool_calls[0].function.name, "get_current_time_and_date");

    assert_eq!(messages[2].role, ChatCompletionMessageRole::Tool);
    assert_eq!(messages[2].tool_call_id, Some("call_123".to_string()));
    assert_eq!(
        messages[2].name,
        Some("get_current_time_and_date".to_string())
    );
}

#[tokio::test]
async fn test_convert_chat_to_messages_tool_calling_fallback() {
    // Create a Chat struct with invalid JSON function call data
    let chat = Chat {
        id: 0,
        conversation_id: 0,
        user_request: "What's the current time in San Francisco?".to_string(),
        tool_calls: Some("[invalid json]".to_string()),
        tool_call_results: Some("[some results]".to_string()),
        prompt: "time query".to_string(),
        prompt_id: 0,
        model_name: "gpt-4".to_string(),
        response: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    };

    // Call convert_chat_to_messages on this struct
    let messages = convert_chat_to_messages(vec![chat]);

    // Assert the fallback behavior
    assert_eq!(messages.len(), 1);
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
        }],
        Default::default(),
    )
    .await;

    dbg!(&messages);

    assert!(messages.len() == 2);

    assert!(messages[0].content == Some("You are a helpful asistant".to_string()));
    assert!(messages[1].content == Some("How are you today?".to_string()));
}
