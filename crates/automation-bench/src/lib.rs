mod gmail;
mod salesforce;
mod store;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
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

#[derive(Deserialize)]
struct ResetRequest {
    task: Option<String>,
}

async fn reset(State(world): State<Arc<World>>, body: Option<Json<ResetRequest>>) -> StatusCode {
    let task = body.and_then(|Json(request)| request.task);
    if !world.reset(task.as_deref()).await {
        return StatusCode::BAD_REQUEST;
    }
    StatusCode::NO_CONTENT
}
