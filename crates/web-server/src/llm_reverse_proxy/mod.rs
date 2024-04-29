pub mod api_chat_stream;
pub mod api_reverse_proxy;
mod prompt;
pub mod sse_chat_enricher;
pub mod ui_chat_stream;
use axum::Router;
use axum_extra::routing::RouterExt;

use axum_extra::routing::TypedPath;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .typed_get(api_chat_stream::chat_generate)
        .typed_post(api_chat_stream::chat_generate)
        .typed_get(api_reverse_proxy::handler)
        .typed_post(api_reverse_proxy::handler)
        .typed_post(ui_chat_stream::chat_generate)
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/completions/:chat_id")]
pub struct UICompletions {
    pub chat_id: i32,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/*path")]
pub struct LLMHandler {
    pub path: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/v1/chat/completions")]
pub struct ApiChatHandler {}
