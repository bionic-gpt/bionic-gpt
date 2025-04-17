use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tiktoken_rs::{num_tokens_from_messages, ChatCompletionRequestMessage};

#[derive(Debug, Deserialize)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: i32,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug)]
pub struct Embedding {
    pub embeddings: Vec<f32>,
    pub text: String,
}

/// Trims the input text to fit within the specified context length.
/// If context_length is zero or negative, a default value will be used.
fn trim_to_context_length(input: &str, context_length: i32, _model: &str) -> String {
    // Handle empty input
    if input.is_empty() {
        return String::new();
    }

    // If context_length is zero or negative, use a default value
    let effective_context_length = if context_length <= 0 {
        256 // Default context length
    } else {
        context_length
    };

    // Create a ChatCompletionRequestMessage to count tokens
    let request = ChatCompletionRequestMessage {
        role: "user".to_string(),
        content: Some(input.to_string()),
        name: None,
        function_call: None,
    };

    // Count tokens using num_tokens_from_messages
    let token_count = num_tokens_from_messages("gpt-4", &[request.clone()]).unwrap() as i32;

    // If token count is within context length, return the original input
    if token_count <= effective_context_length {
        tracing::info!(
            "Input has {} tokens, which is within context length {}",
            token_count,
            effective_context_length
        );
        return input.to_string();
    }

    // If we need to trim, we'll use a binary search approach to find the right cutoff point
    tracing::info!(
        "Input has {} tokens, trimming to context length {}",
        token_count,
        effective_context_length
    );

    // We'll use a different approach to handle Unicode characters correctly
    // Start with a small chunk and gradually increase it until we hit the token limit
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut current_token_count = 0;

    // Start with a small chunk size and increase it gradually
    let mut chunk_size = 1;
    while chunk_size <= chars.len() {
        let chunk: String = chars[0..chunk_size].iter().collect();

        let request = ChatCompletionRequestMessage {
            role: "user".to_string(),
            content: Some(chunk.clone()),
            name: None,
            function_call: None,
        };

        let chunk_token_count = num_tokens_from_messages("gpt-4", &[request]).unwrap() as i32;

        if chunk_token_count <= effective_context_length {
            // This chunk fits, save it and try a larger one
            result = chunk;
            current_token_count = chunk_token_count;
            chunk_size = (chunk_size * 3) / 2; // Increase by 50%

            // Make sure we don't go out of bounds
            if chunk_size > chars.len() {
                chunk_size = chars.len();
            }
        } else {
            // This chunk is too large, try a smaller one
            // If we're already at the smallest size, we can't do anything more
            if chunk_size <= 1 {
                break;
            }

            // Try a smaller chunk size
            chunk_size = (chunk_size * 2) / 3; // Decrease by 33%
            if chunk_size < 1 {
                chunk_size = 1;
            }
        }

        // If we've processed all characters, we're done
        if chunk_size == chars.len() && current_token_count <= effective_context_length {
            break;
        }
    }

    tracing::info!(
        "Trimmed input from {} to approximately {} tokens",
        token_count,
        effective_context_length
    );
    result
}

pub async fn get_embeddings(
    input: &str,
    api_end_point: &str,
    model: &str,
    context_length: i32,
    api_key: &Option<String>,
) -> Result<Vec<f32>, Box<dyn Error>> {
    let client = Client::new();

    // Convert input to UTF-8 and trim to context length
    let text = String::from_utf8_lossy(input.as_bytes()).to_string();
    let trimmed_text = trim_to_context_length(&text, context_length, model);

    let calling_json = EmbeddingRequest {
        input: trimmed_text,
        model: model.to_string(),
        user: None,
    };

    let request = if let Some(api_key) = api_key {
        client
            .post(api_end_point)
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .json(&calling_json)
    } else {
        client.post(api_end_point).json(&calling_json)
    };

    //send request
    let response = request.send().await?;

    if response.status().is_client_error() {
        tracing::error!("Problem with request: {:?}", response);
        let response_text = response.text().await?;
        tracing::error!("{:?}", response_text);
        Err("Problem with request")?
    } else {
        let result = response.json::<EmbeddingResponse>().await?;

        if let Some(result) = result.data.first() {
            tracing::info!("Processing {} bytes", result.embedding.len());
            Ok(result.embedding.clone())
        } else {
            tracing::error!("Problem looking at results from API");
            Err("Problem generating embeddings")?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_to_context_length_empty_input() {
        let input = "";
        let result = trim_to_context_length(input, 100, "gpt-4");
        assert_eq!(result, "");
    }

    #[test]
    fn test_trim_to_context_length_within_limit() {
        let input = "This is a short text that should be within the context length.";
        let result = trim_to_context_length(input, 100, "gpt-4");
        assert_eq!(result, input);
    }

    #[test]
    fn test_trim_to_context_length_zero_context() {
        let input = "This is a test with zero context length.";
        let result = trim_to_context_length(input, 0, "gpt-4");

        // With zero context length, it should use the default (256)
        assert_eq!(result, input);
    }

    #[test]
    fn test_trim_to_context_length_negative_context() {
        let input = "This is a test with negative context length.";
        let result = trim_to_context_length(input, -10, "gpt-4");

        // With negative context length, it should use the default (256)
        assert_eq!(result, input);
    }

    #[test]
    fn test_trim_to_context_length_invalid_model() {
        let input = "This is a test with an invalid model name.";
        let result = trim_to_context_length(input, 100, "invalid-model-name");

        // The model parameter is now ignored, so this should work the same as with a valid model
        assert_eq!(result, input);
    }
}
