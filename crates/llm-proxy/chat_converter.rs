use db::{Chat, ChatRole};
use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};

/// Converts a database chat role to an OpenAI API chat completion message role
pub fn convert_chat_role(db_role: &ChatRole) -> ChatCompletionMessageRole {
    match db_role {
        ChatRole::User => ChatCompletionMessageRole::User,
        ChatRole::Assistant => ChatCompletionMessageRole::Assistant,
        ChatRole::Tool => ChatCompletionMessageRole::Tool,
        ChatRole::System => ChatCompletionMessageRole::System,
        ChatRole::Developer => ChatCompletionMessageRole::Developer,
    }
}

/// Converts a vector of database Chat records to OpenAI API ChatCompletionMessage format
pub fn convert_chat_to_messages(conversation: Vec<Chat>) -> Vec<ChatCompletionMessage> {
    let mut messages: Vec<ChatCompletionMessage> = Default::default();
    for chat in conversation {
        let tool_calls = chat
            .tool_calls
            .map(|tool_calls| serde_json::from_str(&tool_calls).unwrap_or_default());

        messages.push(ChatCompletionMessage {
            role: convert_chat_role(&chat.role),
            content: chat.content,
            tool_call_id: chat.tool_call_id,
            tool_calls,
            name: None,
        });
    }
    messages
}
