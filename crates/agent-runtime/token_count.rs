use rig::message::{AssistantContent, Message, ToolResultContent, UserContent};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

fn message_role_and_content(message: &Message) -> (String, Option<String>) {
    match message {
        Message::User { content } => {
            let first = content.first();
            let content_text = match first {
                UserContent::Text(text) => Some(text.text.clone()),
                UserContent::ToolResult(result) => {
                    result.content.iter().find_map(|item| match item {
                        ToolResultContent::Text(text) => Some(text.text.clone()),
                        ToolResultContent::Image(_) => None,
                    })
                }
                _ => None,
            };
            ("user".to_string(), content_text)
        }
        Message::Assistant { content, .. } => {
            let content_text = content.iter().find_map(|item| match item {
                AssistantContent::Text(text) => Some(text.text.clone()),
                AssistantContent::ToolCall(_) | AssistantContent::Reasoning(_) => None,
            });
            ("assistant".to_string(), content_text)
        }
    }
}

pub fn token_count(messages: Vec<Message>) -> i32 {
    let messages: Vec<ChatCompletionRequestMessage> = messages
        .iter()
        .map(|msg| {
            let (role, content) = message_role_and_content(msg);
            ChatCompletionRequestMessage {
                role,
                content,
                name: None,
                function_call: None,
            }
        })
        .collect();

    num_tokens_from_messages("gpt-4", &messages).unwrap_or(0) as i32
}

pub fn token_count_from_string(message: &str) -> i32 {
    token_count(vec![Message::user(message.to_string())])
}
