mod config;
mod unstructured;

use db::queries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::new();
    dbg!(&config);
    let pool = db::create_pool(&config.app_database_url);
    let client = pool.get().await?;

    loop {
        let unprocessed_docs = queries::documents::unprocessed_documents()
            .bind(&client)
            .all()
            .await?;

        for document in unprocessed_docs {
            let dataset = queries::datasets::pipeline_dataset()
                .bind(&client, &document.dataset_id)
                .one()
                .await?;

            let structured_data = crate::unstructured::document_to_chunks(
                document.content,
                &document.file_name,
                dataset.combine_under_n_chars as u32,
                dataset.new_after_n_chars as u32,
                dataset.multipage_sections,
                &config.unstructured_endpoint,
            )
            .await;

            match structured_data {
                Ok(structured_data) => {
                    for text in structured_data {
                        client
                            .execute(
                                "
                                INSERT INTO chunks (
                                    document_id,
                                    page_number,
                                    text
                                ) 
                                VALUES 
                                    ($1, $2, encrypt_text($3))",
                                &[
                                    &document.id,
                                    &text.metadata.page_number.unwrap_or(0),
                                    &text.text,
                                ],
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

        loop {
            let unprocessed = queries::chunks::unprocessed_chunks()
                .bind(&client)
                .all()
                .await?;

            if unprocessed.is_empty() {
                break;
            }

            if let Some(embedding) = unprocessed.first() {
                let embeddings = embeddings_api::get_embeddings(
                    &embedding.text,
                    &embedding.base_url,
                    &embedding.model,
                    embedding.context_size,
                    &embedding.api_key,
                )
                .await;
                if let Ok(embeddings) = embeddings {
                    let embedding_data = pgvector::Vector::from(embeddings);
                    client
                        .execute(
                            "
                            UPDATE chunks SET (processed, embeddings) = (TRUE, $1)
                            WHERE id = $2
                            ",
                            &[&embedding_data, &embedding.id],
                        )
                        .await?;
                    tracing::info!("Processing embedding id {:?}", embedding.id);
                } else {
                    tracing::info!("Failed to process embedding id {:?}", embedding.id);
                    client
                        .execute(
                            "
                            UPDATE chunks SET processed = TRUE
                            WHERE id = $1
                            ",
                            &[&embedding.id],
                        )
                        .await?;
                }
            }
        }

        // Run this every 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}
