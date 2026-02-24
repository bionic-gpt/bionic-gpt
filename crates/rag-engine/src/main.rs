mod chunks;
mod config;
mod kreuzberg_api;
mod unstructured;

use crate::chunks::ChunkText;
use crate::config::ChunkingEngine;
use db::queries;
use object_storage::StorageConfig;
use rig::client::EmbeddingsClient;
use rig::embeddings::EmbeddingModel;
use rig::providers::{ollama, openai};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::new();
    dbg!(&config);
    let pool = db::create_pool(&config.app_database_url);
    let storage_config = StorageConfig::database(pool.clone());
    let client = pool.get().await?;

    loop {
        // Process unprocessed documents in batches until none remain
        loop {
            let docs = queries::documents::unprocessed_documents()
                .bind(&client, &config.batch_size)
                .all()
                .await?;

            if docs.is_empty() {
                break;
            }

            for document in docs {
                let dataset = queries::datasets::pipeline_dataset()
                    .bind(&client, &document.dataset_id)
                    .one()
                    .await?;

                let bytes = match load_document_bytes(&storage_config, &document).await {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        let error = format!("Not able to load document bytes: {}", error);
                        queries::documents::fail_document()
                            .bind(&client, &error, &document.id)
                            .await?;
                        tracing::error!(error);
                        continue;
                    }
                };

                let structured_data: Result<Vec<ChunkText>, Box<dyn std::error::Error>> =
                    match config.chunking_engine {
                        ChunkingEngine::UnstructuredApi => crate::unstructured::document_to_chunks(
                            bytes,
                            &document.file_name,
                            dataset.combine_under_n_chars as u32,
                            dataset.new_after_n_chars as u32,
                            dataset.multipage_sections,
                            &config.unstructured_endpoint,
                        )
                        .await
                        .map(|chunks| {
                            chunks
                                .into_iter()
                                .map(|chunk| ChunkText {
                                    text: chunk.text,
                                    page_number: chunk.metadata.page_number,
                                })
                                .collect()
                        }),
                        ChunkingEngine::KreuzbergApi => {
                            crate::kreuzberg_api::document_to_chunks(
                                bytes,
                                &document.file_name,
                                dataset.new_after_n_chars as u32,
                                dataset.combine_under_n_chars as u32,
                                &dataset.chunking_strategy,
                                &config.kreuzberg_endpoint,
                            )
                            .await
                        }
                    };

                match structured_data {
                    Ok(structured_data) => {
                        for text in structured_data {
                            client
                                .execute(
                                    "
                                INSERT INTO rag.chunks (
                                    document_id,
                                    page_number,
                                    text
                                )
                                VALUES
                                    ($1, $2, encrypt_text($3))",
                                    &[&document.id, &text.page_number.unwrap_or(0), &text.text],
                                )
                                .await?;
                        }
                    }
                    Err(error) => {
                        let error = format!("Not able to parse document {}", error);
                        queries::documents::fail_document()
                            .bind(&client, &error, &document.id)
                            .await?;

                        tracing::error!(error);
                    }
                }
            }
        }

        // Process embeddings in batches until none remain
        loop {
            let unprocessed = queries::chunks::unprocessed_chunks()
                .bind(&client, &config.batch_size)
                .all()
                .await?;

            if unprocessed.is_empty() {
                break;
            }

            for embedding in unprocessed {
                match get_embeddings_via_rig(
                    &embedding.text,
                    &embedding.base_url,
                    &embedding.model,
                    embedding.context_size,
                    embedding.api_key.as_deref(),
                )
                .await
                {
                    Ok(embeddings) => {
                        let embedding_data = pgvector::Vector::from(embeddings);
                        client
                            .execute(
                                "
                                UPDATE rag.chunks SET (processed, embeddings) = (TRUE, $1)
                                WHERE id = $2
                                ",
                                &[&embedding_data, &embedding.id],
                            )
                            .await?;
                        tracing::info!("Processing embedding id {:?}", embedding.id);
                    }
                    Err(error) => {
                        tracing::error!(
                            "Failed to process embedding id {:?}: {:?}",
                            embedding.id,
                            error
                        );
                        client
                            .execute(
                                "
                                UPDATE rag.chunks SET processed = TRUE
                                WHERE id = $1
                                ",
                                &[&embedding.id],
                            )
                            .await?;
                    }
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn load_document_bytes(
    storage_config: &StorageConfig,
    document: &db::queries::documents::UnprocessedDocument,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(content) = &document.content {
        return Ok(content.clone());
    }

    if let Some(object_id) = document.object_id {
        let object = object_storage::get(storage_config, object_id).await?;
        if let Some(bytes) = object.object_data {
            return Ok(bytes);
        }
    }

    Err("document bytes missing".into())
}

fn trim_to_context_length(input: &str, context_length: i32) -> String {
    if input.is_empty() {
        return String::new();
    }
    let effective_context_length = if context_length <= 0 {
        256
    } else {
        context_length
    };
    let char_count = input.chars().count() as i32;
    if char_count <= effective_context_length {
        return input.to_string();
    }
    input
        .chars()
        .take(effective_context_length as usize)
        .collect()
}

async fn get_embeddings_via_rig(
    input: &str,
    api_end_point: &str,
    model: &str,
    context_length: i32,
    api_key: Option<&str>,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let text = String::from_utf8_lossy(input.as_bytes()).to_string();
    let trimmed_text = trim_to_context_length(&text, context_length);

    let normalized_base_url = api_end_point
        .strip_suffix("/embeddings")
        .or_else(|| api_end_point.strip_suffix("/v1/embeddings"))
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| api_end_point.trim_end_matches('/').to_string());

    let embedding = if let Some(key) = api_key.filter(|k| !k.trim().is_empty()) {
        let client = openai::Client::builder(key)
            .base_url(&normalized_base_url)
            .build();
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await?
    } else {
        let client = ollama::Client::builder()
            .base_url(&normalized_base_url)
            .build();
        client
            .embedding_model(model)
            .embed_text(&trimmed_text)
            .await?
    };

    Ok(embedding.vec.into_iter().map(|v| v as f32).collect())
}
