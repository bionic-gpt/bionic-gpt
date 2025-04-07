use super::{Completion, Message};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

pub async fn token_count(completion: &Completion) -> i32 {
    let messages: Vec<ChatCompletionRequestMessage> = completion
        .messages
        .iter()
        .map(|msg| ChatCompletionRequestMessage {
            role: msg.role.clone(),
            content: Some(msg.content.clone()),
            name: None,
            function_call: None,
        })
        .collect();

    num_tokens_from_messages("gpt-4", &messages).unwrap() as i32
}

pub async fn token_count_from_string(message: &str) -> i32 {
    let completion = Completion {
        model: "".to_string(),
        max_tokens: None,
        stream: None,
        messages: vec![super::Message {
            role: "".to_string(),
            content: message.to_string(),
            tool_call_id: None,
            tool_calls: None,
            name: None,
        }],
        temperature: None,
        tools: None,
        tool_choice: None,
    };

    token_count(&completion).await
}

pub async fn token_count_from_messages(messages: Vec<Message>) -> i32 {
    let completion = Completion {
        model: "".to_string(),
        max_tokens: None,
        stream: None,
        messages,
        temperature: None,
        tools: None,
        tool_choice: None,
    };

    token_count(&completion).await
}
