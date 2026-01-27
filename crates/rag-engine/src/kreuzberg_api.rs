use crate::chunks::ChunkText;
use db::types::public::ChunkingStrategy;
use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ExtractionResult {
    content: String,
}

#[derive(Debug, Serialize)]
struct ChunkRequest {
    text: String,
    chunker_type: String,
    config: ChunkConfig,
}

#[derive(Debug, Serialize)]
struct ChunkConfig {
    max_characters: u32,
    overlap: u32,
    trim: bool,
}

#[derive(Debug, Deserialize)]
struct ChunkResponse {
    chunks: Vec<ChunkItem>,
}

#[derive(Debug, Deserialize)]
struct ChunkItem {
    content: String,
    first_page: Option<usize>,
}

pub async fn document_to_chunks(
    bytes: Vec<u8>,
    file_name: &str,
    chunk_size: u32,
    chunk_overlap: u32,
    chunking_strategy: &ChunkingStrategy,
    kreuzberg_endpoint: &str,
) -> Result<Vec<ChunkText>, Box<dyn Error>> {
    let client = Client::new();
    let extract_url = format!("{}/extract", kreuzberg_endpoint);

    let file_part = multipart::Part::bytes(bytes).file_name(file_name.to_string());
    let form = multipart::Form::new().part("files", file_part);

    let extract_response = client.post(extract_url).multipart(form).send().await?;
    if !extract_response.status().is_success() {
        let body = extract_response.text().await?;
        return Err(format!("kreuzberg extract failed: {}", body).into());
    }

    let extracted: Vec<ExtractionResult> = extract_response.json().await?;
    let content = extracted
        .first()
        .ok_or("kreuzberg extract returned no results")?
        .content
        .to_string();

    let chunker_type = match chunking_strategy {
        ChunkingStrategy::ByTitle => "markdown",
    };

    let chunk_url = format!("{}/chunk", kreuzberg_endpoint);
    let chunk_request = ChunkRequest {
        text: content,
        chunker_type: chunker_type.to_string(),
        config: ChunkConfig {
            max_characters: chunk_size,
            overlap: chunk_overlap,
            trim: true,
        },
    };

    let chunk_response = client.post(chunk_url).json(&chunk_request).send().await?;
    if !chunk_response.status().is_success() {
        let body = chunk_response.text().await?;
        return Err(format!("kreuzberg chunk failed: {}", body).into());
    }

    let chunk_response: ChunkResponse = chunk_response.json().await?;
    let chunks = chunk_response
        .chunks
        .into_iter()
        .map(|chunk| ChunkText {
            text: chunk.content,
            page_number: chunk.first_page.map(|page| page as i32),
        })
        .collect();

    Ok(chunks)
}
