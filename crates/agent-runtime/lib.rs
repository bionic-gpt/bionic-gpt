pub mod api_chat_orchestrator;
mod chat_converter;
mod chat_types;
mod context_builder;
mod errors;
mod jwt;
pub mod limits;
pub mod moderation;
pub mod provider_passthrough;
pub mod stream_assembler;
pub mod stream_errors;
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
        .typed_get(api_chat_orchestrator::chat_generate)
        .typed_post(api_chat_orchestrator::chat_generate)
        .typed_post(synthesize::synthesize)
        .typed_get(provider_passthrough::handler)
        .typed_post(provider_passthrough::handler)
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

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/{*path}")]
pub struct LLMHandler {
    pub path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/chat/completions")]
pub struct ApiChatHandler {}
