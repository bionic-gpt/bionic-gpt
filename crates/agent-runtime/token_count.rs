use openai_api::{ChatCompletionMessage, ChatCompletionMessageRole};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

pub fn token_count(messages: Vec<ChatCompletionMessage>) -> i32 {
    let messages: Vec<ChatCompletionRequestMessage> = messages
        .iter()
        .map(|msg| ChatCompletionRequestMessage {
            role: match msg.role {
                ChatCompletionMessageRole::System => "system",
                ChatCompletionMessageRole::User => "user",
                ChatCompletionMessageRole::Assistant => "assistant",
                ChatCompletionMessageRole::Function => "function",
                ChatCompletionMessageRole::Tool => "tool",
                ChatCompletionMessageRole::Developer => "developer",
            }
            .to_string(),
            content: msg.content.clone(),
            name: None,
            function_call: None,
        })
        .collect();

    num_tokens_from_messages("gpt-4", &messages).unwrap_or(0) as i32
}

pub fn token_count_from_string(message: &str) -> i32 {
    token_count(vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(message.to_string()),
        tool_call_id: None,
        tool_calls: None,
        name: None,
    }])
}
