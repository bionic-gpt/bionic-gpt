mod gmail;
mod salesforce;
mod store;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use store::World;

pub fn app() -> Router {
    app_with_state(Arc::new(World::default()))
}

pub fn app_with_state(world: Arc<World>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/benchmark/reset", post(reset))
        .nest("/api/gmail", gmail::routes())
        .nest("/api/salesforce", salesforce::routes())
        .with_state(world)
}

async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn reset(State(world): State<Arc<World>>) -> StatusCode {
    world.reset().await;
    StatusCode::NO_CONTENT
}
