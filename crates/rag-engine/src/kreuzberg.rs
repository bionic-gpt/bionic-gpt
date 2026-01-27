use crate::chunks::ChunkText;
use kreuzberg::{detect_mime_type_from_bytes, extract_bytes, ChunkingConfig, ExtractionConfig};
use std::error::Error;

pub async fn document_to_chunks(
    bytes: Vec<u8>,
    chunk_size: u32,
    chunk_overlap: u32,
) -> Result<Vec<ChunkText>, Box<dyn Error>> {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_chars: chunk_size as usize,
            max_overlap: chunk_overlap as usize,
            embedding: None,
            preset: None,
        }),
        ..Default::default()
    };

    let mime_type = detect_mime_type_from_bytes(&bytes)?;
    let result = extract_bytes(&bytes, &mime_type, &config).await?;

    let chunks = if let Some(extracted_chunks) = result.chunks {
        extracted_chunks
            .into_iter()
            .map(|chunk| ChunkText {
                text: chunk.content,
                page_number: chunk.metadata.first_page.map(|page| page as i32),
            })
            .collect()
    } else {
        vec![ChunkText {
            text: result.content,
            page_number: None,
        }]
    };

    Ok(chunks)
}
