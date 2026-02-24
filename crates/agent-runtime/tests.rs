use db::{Chat, ChatRole, ChatStatus};
use rig::message::{AssistantContent, Message, UserContent};
use rig::OneOrMany;
use time::OffsetDateTime;

use crate::context_builder::{convert_chat_to_messages, generate_prompt};
use crate::moderation::strip_tool_data;

#[tokio::test]
async fn test_convert_chat_to_messages_tool_calling_fallback() {
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

    let messages = convert_chat_to_messages(vec![chat]);
    assert_eq!(messages.len(), 1);
}

#[tokio::test]
async fn test_generate_prompt() {
    let messages = generate_prompt(
        2048,
        1024,
        1.0,
        Some("You are a helpful asistant".to_string()),
        vec![Message::user("How are you today?")],
    )
    .await;

    assert_eq!(messages.len(), 2);
    assert_eq!(
        text_content(&messages[0]),
        Some("You are a helpful asistant")
    );
    assert_eq!(text_content(&messages[1]), Some("How are you today?"));
}

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

fn is_user(msg: &Message) -> bool {
    matches!(msg, Message::User { .. })
}

fn is_assistant(msg: &Message) -> bool {
    matches!(msg, Message::Assistant { .. })
}

fn text_content(msg: &Message) -> Option<&str> {
    match msg {
        Message::User { content } => content.iter().find_map(|c| match c {
            UserContent::Text(text) => Some(text.text.as_str()),
            _ => None,
        }),
        Message::Assistant { content, .. } => content.iter().find_map(|c| match c {
            AssistantContent::Text(text) => Some(text.text.as_str()),
            _ => None,
        }),
    }
}

fn assistant_tool_calls(msg: &Message) -> Vec<(String, String, serde_json::Value)> {
    match msg {
        Message::Assistant { content, .. } => content
            .iter()
            .filter_map(|c| match c {
                AssistantContent::ToolCall(tc) => Some((
                    tc.id.clone(),
                    tc.function.name.clone(),
                    tc.function.arguments.clone(),
                )),
                _ => None,
            })
            .collect(),
        _ => vec![],
    }
}

fn tool_result_call_id(msg: &Message) -> Option<&str> {
    match msg {
        Message::User { content } => content.iter().find_map(|c| match c {
            UserContent::ToolResult(res) => Some(res.id.as_str()),
            _ => None,
        }),
        _ => None,
    }
}

#[tokio::test]
async fn test_complete_time_conversation_flow() {
    let conversation = vec![
        create_test_chat(69, ChatRole::User, Some("What time is it".to_string()), None, None),
        create_test_chat(
            70,
            ChatRole::Assistant,
            None,
            Some(create_tool_call_json(
                "call_a96p",
                "get_current_time_and_date",
                r#"{"format":"human_readable"}"#,
            )),
            None,
        ),
        create_test_chat(
            71,
            ChatRole::Tool,
            Some(r#"{"current_time":"2025-05-31 08:53:33 UTC","format":"human_readable","timestamp":1748681613,"timezone":"utc"}"#.to_string()),
            None,
            Some("call_a96p".to_string()),
        ),
        create_test_chat(
            72,
            ChatRole::Assistant,
            Some("The current time is 2025-05-31 08:53:33 UTC.".to_string()),
            None,
            None,
        ),
        create_test_chat(
            73,
            ChatRole::User,
            Some("How about in Bangkok?".to_string()),
            None,
            None,
        ),
    ];

    let messages = convert_chat_to_messages(conversation);
    assert_eq!(messages.len(), 5);
    assert!(is_user(&messages[0]));
    assert!(is_assistant(&messages[1]));
    assert_eq!(tool_result_call_id(&messages[2]), Some("call_a96p"));
    assert!(is_assistant(&messages[3]));
}

#[tokio::test]
async fn test_second_time_conversation_flow() {
    let conversation = vec![
        create_test_chat(74, ChatRole::User, Some("Whats the time?".to_string()), None, None),
        create_test_chat(
            75,
            ChatRole::Assistant,
            None,
            Some(create_tool_call_json(
                "call_yc5v",
                "get_current_time_and_date",
                r#"{"format":"human_readable"}"#,
            )),
            None,
        ),
        create_test_chat(
            76,
            ChatRole::Tool,
            Some(r#"{"current_time":"2025-05-31 10:42:37 UTC","format":"human_readable","timestamp":1748688157,"timezone":"utc"}"#.to_string()),
            None,
            Some("call_yc5v".to_string()),
        ),
        create_test_chat(
            77,
            ChatRole::Assistant,
            Some("The current time is 10:42:37 UTC on May 31, 2025.".to_string()),
            None,
            None,
        ),
    ];

    let messages = convert_chat_to_messages(conversation);
    assert_eq!(messages.len(), 4);
    assert!(is_user(&messages[0]));
    assert!(is_assistant(&messages[1]));
    assert_eq!(tool_result_call_id(&messages[2]), Some("call_yc5v"));
    assert!(is_assistant(&messages[3]));
}

#[tokio::test]
async fn test_tool_call_json_parsing() {
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
    let tool_calls = assistant_tool_calls(&messages[0]);
    assert_eq!(tool_calls.len(), 1);
    assert_eq!(tool_calls[0].0, "call_test123");
    assert_eq!(tool_calls[0].1, "test_function");
}

#[tokio::test]
async fn test_tool_call_id_linking() {
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
    assert_eq!(assistant_tool_calls(&messages[0])[0].0, "call_link_test");
    assert_eq!(tool_result_call_id(&messages[1]), Some("call_link_test"));
}

#[tokio::test]
async fn test_history_truncation_keeps_latest() {
    use crate::token_count::token_count;

    fn mk_msg(content: &str) -> Message {
        Message::user(content.to_string())
    }

    let small_msg = mk_msg("hi");
    let small_tokens = token_count(vec![small_msg.clone()]) as usize;

    let large_content = "long ".repeat(100);
    let large_msg = mk_msg(&large_content);
    let large_tokens = token_count(vec![large_msg.clone()]) as usize;

    let context_size = large_tokens + small_tokens * 4 + 1;

    let history = vec![
        mk_msg("m1"),
        mk_msg("m2"),
        mk_msg("m3"),
        mk_msg("m4"),
        mk_msg("m5"),
        large_msg.clone(),
    ];

    let messages = generate_prompt(context_size, 0, 1.0, None, history).await;

    let contents: Vec<_> = messages.iter().filter_map(text_content).collect();
    assert_eq!(messages.len(), 4);
    assert_eq!(contents[0], "m3");
    assert_eq!(contents[1], "m4");
    assert_eq!(contents[2], "m5");
    assert_eq!(contents[3], large_content);
}

#[test]
fn test_strip_tool_data_removes_tool_messages() {
    let messages = vec![
        Message::user("hi"),
        Message::Assistant {
            id: None,
            content: OneOrMany::many(vec![AssistantContent::ToolCall(rig::message::ToolCall {
                id: "call1".to_string(),
                call_id: None,
                function: rig::message::ToolFunction {
                    name: "do_it".to_string(),
                    arguments: serde_json::json!({}),
                },
            })])
            .unwrap(),
        },
        Message::tool_result_with_call_id("call1", None, "{}"),
    ];

    let sanitized = strip_tool_data(&messages);
    assert_eq!(sanitized.len(), 1);
    assert!(is_user(&sanitized[0]));
    assert_eq!(text_content(&sanitized[0]), Some("hi"));
}

#[test]
fn test_strip_tool_data_changes_system_to_user() {
    let messages = vec![Message::user("hi")];
    let sanitized = strip_tool_data(&messages);
    assert_eq!(sanitized.len(), 1);
    assert!(is_user(&sanitized[0]));
    assert_eq!(text_content(&sanitized[0]), Some("hi"));
}
