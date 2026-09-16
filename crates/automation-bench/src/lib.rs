use axum::{http::StatusCode, routing::post, Router};

pub fn app() -> Router {
    Router::new().route("/benchmark/reset", post(reset))
}

async fn reset() -> StatusCode {
    StatusCode::NO_CONTENT
}
