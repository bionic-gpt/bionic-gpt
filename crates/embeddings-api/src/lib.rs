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

#[derive(Serialize, Debug)]
struct CallingJson {
    input: String,
    model: String,
}

#[derive(Debug)]
pub struct Embedding {
    pub embeddings: Vec<f32>,
    pub text: String,
}

pub async fn get_embeddings(
    input: &str,
    api_end_point: &str,
    model: &str,
) -> Result<Vec<f32>, Box<dyn Error>> {
    let client = Client::new();

    let text = String::from_utf8_lossy(input.as_bytes()).to_string();
    let calling_json = EmbeddingRequest {
        input: text.clone(),
        model: model.to_string(),
        user: None,
    };

    //send request
    let response = client
        .post(api_end_point)
        .json(&calling_json)
        .send()
        .await?;
    let result = response.json::<EmbeddingResponse>().await?;

    if let Some(result) = result.data.first() {
        tracing::info!("Processing {} bytes", result.embedding.len());
        Ok(result.embedding.clone())
    } else {
        tracing::error!("Problem looking at results from API");
        Err("Problem generating embeddings")?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_embeddings() {
        let input = "The food was delicious and the waiter...".to_string();
        let embeddings = get_embeddings(
            &input,
            "http://embeddings-api:80/embeddings",
            "text-embedding-ada-002",
        )
        .await
        .unwrap();

        println!("{:#?}", embeddings);
    }
}
