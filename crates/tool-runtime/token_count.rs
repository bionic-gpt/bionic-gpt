use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

pub fn token_count_from_string(message: &str) -> i32 {
    let messages = vec![ChatCompletionRequestMessage {
        role: "user".to_string(),
        content: Some(message.to_string()),
        name: None,
        function_call: None,
    }];

    num_tokens_from_messages("gpt-4", &messages).unwrap_or(0) as i32
}
