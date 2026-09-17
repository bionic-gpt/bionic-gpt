use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = env::var("INTEGRATION_SIMULATOR_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()?;

    axum::serve(
        tokio::net::TcpListener::bind(address).await?,
        integration_simulator::app(),
    )
    .await?;
    Ok(())
}
