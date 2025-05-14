use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
/// Trims the input text to fit within the specified context length.
/// If context_length is zero or negative, a default value will be used.
/// Uses a simple 1:1 character-to-token ratio for efficiency.
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

    // Get the character count
    let char_count = input.chars().count() as i32;

    // If character count is within context length, return the original input
    if char_count <= effective_context_length {
        tracing::info!(
            "Input has {} characters, which is within context length {}",
            char_count,
            effective_context_length
        );
        return input.to_string();
    }

    // If we need to trim, simply truncate the string
    tracing::info!(
        "Input has {} characters, trimming to context length {}",
        char_count,
        effective_context_length
    );

    // Collect characters up to the effective_context_length
    let result: String = input
        .chars()
        .take(effective_context_length as usize)
        .collect();

    tracing::info!(
        "Trimmed input from {} to {} characters",
        char_count,
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
