pub mod api_reverse_proxy;
pub mod ui_completions;
use axum::Router;
use axum_extra::routing::RouterExt;

use axum_extra::routing::TypedPath;
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .typed_get(api_reverse_proxy::handler)
        .typed_post(api_reverse_proxy::handler)
        .typed_post(ui_completions::handler)
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
