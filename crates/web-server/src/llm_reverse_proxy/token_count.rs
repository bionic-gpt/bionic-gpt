use super::Completion;
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
