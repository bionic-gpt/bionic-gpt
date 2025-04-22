use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

pub fn token_count(messages: Vec<ChatCompletionMessage>) -> i32 {
    let messages: Vec<ChatCompletionRequestMessage> = messages
        .iter()
        .map(|msg| ChatCompletionRequestMessage {
            role: "user".to_string(),
            content: msg.content.clone(),
            name: None,
            function_call: None,
        })
        .collect();

    num_tokens_from_messages("gpt-4", &messages).unwrap() as i32
}

pub fn token_count_from_string(message: &str) -> i32 {
    token_count(vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(message.to_string()),
        tool_call_id: None,
        tool_calls: None,
        name: None,
        function_call: None,
    }])
}
