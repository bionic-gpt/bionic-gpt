pub mod api_chat_stream;
pub mod api_reverse_proxy;
mod errors;
pub mod function_executor;
pub mod function_tools;
mod jwt;
pub mod limits;
mod prompt;
pub mod sse_chat_enricher;
pub mod sse_chat_error;
pub mod synthesize;
pub mod token_count;
pub mod tool_call_handler;
pub mod ui_chat_stream;
use axum::Router;
use axum_extra::routing::RouterExt;
use tower_http::cors::{Any, CorsLayer};

use axum_extra::routing::TypedPath;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema object
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub r#type: String, // "function"
    pub function: FunctionDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String, // "function"
    pub function: ToolCallFunction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub role: String, // "tool"
    pub name: String,
    pub content: String, // Result as string
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
}

pub fn routes() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows requests from any origin
        .allow_methods(Any) // Allows any HTTP method
        .allow_headers(Any); // Allows any header

    Router::new()
        .typed_get(api_chat_stream::chat_generate)
        .typed_post(api_chat_stream::chat_generate)
        .typed_post(synthesize::synthesize)
        .typed_get(api_reverse_proxy::handler)
        .typed_post(api_reverse_proxy::handler)
        .typed_post(ui_chat_stream::chat_generate)
        .typed_get(ui_chat_stream::chat_generate)
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
