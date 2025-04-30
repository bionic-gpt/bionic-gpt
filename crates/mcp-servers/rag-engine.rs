use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("RAG Engine Server: Hello World!");

    // Keep the server running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
