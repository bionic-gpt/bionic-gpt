use db::{Chat, ChatRole, ChatStatus};
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};
use time::OffsetDateTime;

use crate::{chat_converter::convert_chat_to_messages, prompt::generate_prompt};

#[tokio::test]
async fn test_convert_chat_to_messages_tool_calling_fallback() {
    // Create a Chat struct with invalid JSON function call data
    let chat = Chat {
        role: ChatRole::User,
        id: 0,
        conversation_id: 0,
        content: Some("What's the current time in San Francisco?".to_string()),
        tool_call_id: None,
        tool_calls: Some("[invalid json]".to_string()),
        prompt_id: 0,
        model_name: "gpt-4".to_string(),
        attachments: None,
        status: ChatStatus::Pending,
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

// ============================================================================
// COMPREHENSIVE CHAT CONVERTER TESTS
// Based on provided chat entries for complete conversation flow testing
// ============================================================================

// Test helper functions
fn create_test_chat(
    id: i32,
    role: ChatRole,
    content: Option<String>,
    tool_calls: Option<String>,
    tool_call_id: Option<String>,
) -> Chat {
    Chat {
        id,
        conversation_id: 1,
        role,
        content,
        tool_call_id,
        tool_calls,
        prompt_id: 1,
        model_name: "gpt-4".to_string(),
        attachments: None,
        status: ChatStatus::Success,
        created_at: OffsetDateTime::UNIX_EPOCH,
        updated_at: OffsetDateTime::UNIX_EPOCH,
    }
}

fn create_tool_call_json(id: &str, function_name: &str, arguments: &str) -> String {
    serde_json::json!([{
        "id": id,
        "type": "function",
        "function": {
            "name": function_name,
            "arguments": arguments
        }
    }])
    .to_string()
}

fn assert_message_properties(
    message: &ChatCompletionMessage,
    expected_role: ChatCompletionMessageRole,
    expected_content: Option<&str>,
    expected_tool_call_id: Option<&str>,
    has_tool_calls: bool,
) {
    assert_eq!(message.role, expected_role);
    assert_eq!(message.content.as_deref(), expected_content);
    assert_eq!(message.tool_call_id.as_deref(), expected_tool_call_id);
    assert_eq!(message.tool_calls.is_some(), has_tool_calls);
    assert_eq!(message.name, None); // Always None in current implementation
}

// ============================================================================
// COMPLETE CONVERSATION FLOW TESTS
// ============================================================================

#[tokio::test]
async fn test_complete_time_conversation_flow() {
    // Test the exact conversation flow from provided chat entries
    // Chat ID 69: User asks "What time is it"
    // Chat ID 70: Assistant responds with tool calls
    // Chat ID 71: Tool provides time data with call_a96p ID
    // Chat ID 72: Assistant gives final formatted response

    let conversation = vec![
        create_test_chat(
            69i32,
            ChatRole::User,
            Some("What time is it".to_string()),
            None,
            None,
        ),
        create_test_chat(
            70i32,
            ChatRole::Assistant,
            None,
            Some(create_tool_call_json(
                "call_a96p",
                "get_current_time_and_date",
                r#"{"format":"human_readable"}"#
            )),
            None,
        ),
        create_test_chat(
            71i32,
            ChatRole::Tool,
            Some(r#"{"current_time":"2025-05-31 08:53:33 UTC","format":"human_readable","timestamp":1748681613,"timezone":"utc"}"#.to_string()),
            None,
            Some("call_a96p".to_string()),
        ),
        create_test_chat(
            72i32,
            ChatRole::Assistant,
            Some("The current time is 2025-05-31 08:53:33 UTC.".to_string()),
            None,
            None,
        ),
        create_test_chat(
            72i32,
            ChatRole::User,
            Some("How about in Bangkok?".to_string()),
            None,
            None,
        ),
    ];

    let messages = convert_chat_to_messages(conversation);

    dbg!(&messages);

    // Verify we have 4 messages
    assert_eq!(messages.len(), 4);

    // Verify User message (Chat ID 69)
    assert_message_properties(
        &messages[0],
        ChatCompletionMessageRole::User,
        Some("What time is it"),
        None,
        false,
    );

    // Verify Assistant message with tool calls (Chat ID 70)
    assert_message_properties(
        &messages[1],
        ChatCompletionMessageRole::Assistant,
        None,
        None,
        true,
    );

    let tool_calls = messages[1].tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].id, "call_a96p");
    assert_eq!(tool_calls[0].r#type, "function");
    assert_eq!(tool_calls[0].function.name, "get_current_time_and_date");
    assert_eq!(
        tool_calls[0].function.arguments,
        r#"{"format":"human_readable"}"#
    );

    // Verify Tool message (Chat ID 71)
    assert_message_properties(
        &messages[2],
        ChatCompletionMessageRole::Tool,
        Some(
            r#"{"current_time":"2025-05-31 08:53:33 UTC","format":"human_readable","timestamp":1748681613,"timezone":"utc"}"#,
        ),
        Some("call_a96p"),
        false,
    );

    // Verify final Assistant message (Chat ID 72)
    assert_message_properties(
        &messages[3],
        ChatCompletionMessageRole::Assistant,
        Some("The current time is 2025-05-31 08:53:33 UTC."),
        None,
        false,
    );
}

#[tokio::test]
async fn test_second_time_conversation_flow() {
    // Test the second conversation about time from provided chat entries
    // Chat ID 74: User asks "Whats the time?"
    // Chat ID 75: Assistant responds with tool calls
    // Chat ID 76: Tool provides time data with call_yc5v ID
    // Chat ID 77: Assistant gives final formatted response

    let conversation = vec![
        create_test_chat(
            74i32,
            ChatRole::User,
            Some("Whats the time?".to_string()),
            None,
            None,
        ),
        create_test_chat(
            75i32,
            ChatRole::Assistant,
            None,
            Some(create_tool_call_json(
                "call_yc5v",
                "get_current_time_and_date",
                r#"{"format":"human_readable"}"#
            )),
            None,
        ),
        create_test_chat(
            76i32,
            ChatRole::Tool,
            Some(r#"{"current_time":"2025-05-31 10:42:37 UTC","format":"human_readable","timestamp":1748688157,"timezone":"utc"}"#.to_string()),
            None,
            Some("call_yc5v".to_string()),
        ),
        create_test_chat(
            77i32,
            ChatRole::Assistant,
            Some("The current time is 10:42:37 UTC on May 31, 2025.".to_string()),
            None,
            None,
        ),
    ];

    let messages = convert_chat_to_messages(conversation);

    // Verify we have 4 messages
    assert_eq!(messages.len(), 4);

    // Verify User message
    assert_message_properties(
        &messages[0],
        ChatCompletionMessageRole::User,
        Some("Whats the time?"),
        None,
        false,
    );

    // Verify Assistant message with tool calls
    assert_message_properties(
        &messages[1],
        ChatCompletionMessageRole::Assistant,
        None,
        None,
        true,
    );

    let tool_calls = messages[1].tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].id, "call_yc5v");
    assert_eq!(tool_calls[0].function.name, "get_current_time_and_date");

    // Verify Tool message
    assert_message_properties(
        &messages[2],
        ChatCompletionMessageRole::Tool,
        Some(
            r#"{"current_time":"2025-05-31 10:42:37 UTC","format":"human_readable","timestamp":1748688157,"timezone":"utc"}"#,
        ),
        Some("call_yc5v"),
        false,
    );

    // Verify final Assistant message with different format
    assert_message_properties(
        &messages[3],
        ChatCompletionMessageRole::Assistant,
        Some("The current time is 10:42:37 UTC on May 31, 2025."),
        None,
        false,
    );
}

// ============================================================================
// TOOL CALL PROCESSING TESTS
// ============================================================================

#[tokio::test]
async fn test_tool_call_json_parsing() {
    // Test proper parsing of tool calls JSON
    let conversation = vec![create_test_chat(
        1,
        ChatRole::Assistant,
        None,
        Some(create_tool_call_json(
            "call_test123",
            "test_function",
            r#"{"param1":"value1","param2":42}"#,
        )),
        None,
    )];

    let messages = convert_chat_to_messages(conversation);

    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].role, ChatCompletionMessageRole::Assistant);
    assert!(messages[0].tool_calls.is_some());

    let tool_calls = messages[0].tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].id, "call_test123");
    assert_eq!(tool_calls[0].r#type, "function");
    assert_eq!(tool_calls[0].function.name, "test_function");
    assert_eq!(
        tool_calls[0].function.arguments,
        r#"{"param1":"value1","param2":42}"#
    );
}

#[tokio::test]
async fn test_tool_call_id_linking() {
    // Test tool_call_id properly links Assistant and Tool messages
    let conversation = vec![
        create_test_chat(
            1,
            ChatRole::Assistant,
            None,
            Some(create_tool_call_json(
                "call_link_test",
                "linked_function",
                r#"{"test":true}"#,
            )),
            None,
        ),
        create_test_chat(
            2,
            ChatRole::Tool,
            Some(r#"{"result":"success"}"#.to_string()),
            None,
            Some("call_link_test".to_string()),
        ),
    ];

    let messages = convert_chat_to_messages(conversation);

    assert_eq!(messages.len(), 2);

    // Assistant message has tool_calls populated
    assert!(messages[0].tool_calls.is_some());
    let tool_calls = messages[0].tool_calls.as_ref().unwrap();
    assert_eq!(tool_calls[0].id, "call_link_test");

    // Tool message has correct tool_call_id
    assert_eq!(messages[1].tool_call_id, Some("call_link_test".to_string()));
    assert_eq!(messages[1].role, ChatCompletionMessageRole::Tool);
}
