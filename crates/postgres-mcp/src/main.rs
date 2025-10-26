mod auth;
mod db;
mod error;
mod extractors;
mod handlers;
mod models;
mod sql_utils;
mod state;

use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

use crate::{error::ApiResult, state::AppState};

const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> ApiResult<()> {
    init_tracing();

    let state = AppState::new();
    let app = app_router(state.clone());

    let addr: SocketAddr = DEFAULT_BIND_ADDR
        .parse()
        .expect("DEFAULT_BIND_ADDR must be a valid socket address");
    info!("postgres-mcp server listening on {addr}");

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .await
        .map_err(|err| err.into())
}

fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_spec))
        .nest("/v1", handlers::v1_router())
        .with_state(state)
}

async fn openapi_spec(State(state): State<AppState>) -> &'static str {
    state.openapi_spec()
}

fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,postgres_mcp=debug"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .compact()
        .init();
}
