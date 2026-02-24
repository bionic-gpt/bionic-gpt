mod chat_converter;
mod context_builder;
mod errors;
mod jwt;
pub mod limits;
pub mod moderation;
mod result_sink;
pub mod synthesize;
#[cfg(test)]
mod tests;
mod token_count;
pub mod ui_chat_orchestrator;
#[cfg(test)]
mod ui_chat_orchestrator_tests;
pub mod user_config;
use axum::Router;
use axum_extra::routing::RouterExt;
use tower_http::cors::{Any, CorsLayer};

use axum_extra::routing::TypedPath;
use serde::Deserialize;

pub use user_config::UserConfig;

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows requests from any origin
        .allow_methods(Any) // Allows any HTTP method
        .allow_headers(Any); // Allows any header

    Router::new()
        .typed_post(synthesize::synthesize)
        .typed_post(ui_chat_orchestrator::chat_generate)
        .typed_get(ui_chat_orchestrator::chat_generate)
        .layer(cors) // Apply the CORS layer
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/completions/{chat_id}")]
pub struct UICompletions {
    pub chat_id: i32,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/app/synthesize")]
pub struct UISynthesize {}
