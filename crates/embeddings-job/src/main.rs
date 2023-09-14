mod config;

use db::queries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::new();
    let pool = db::create_pool(&config.app_database_url);
    let client = pool.get().await?;

    loop {
        let unprocessed = queries::embeddings::unprocessed_embeddings()
            .bind(&client)
            .all()
            .await?;

        for embedding in unprocessed {
            let embeddings = open_api::get_embeddings(&embedding.text).await;
            if let Ok(embeddings) = embeddings {
                let embedding_data = pgvector::Vector::from(embeddings);
                client
                    .execute(
                        "
                        UPDATE embeddings SET (processed, embeddings) = (TRUE, $1)
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
                        UPDATE embeddings SET processed = TRUE
                        WHERE id = $1
                        ",
                        &[&embedding.id],
                    )
                    .await?;
            }
        }

        // Run this every 5 seconds
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}
