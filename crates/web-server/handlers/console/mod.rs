mod conversation;
mod delete;
mod generated_output_canvas;
mod generated_output_download;
mod index;
mod send_message;
mod set_default_prompt;
mod update_response;
mod utils;

use axum::{extract::DefaultBodyLimit, Router};
use axum_extra::routing::RouterExt;
pub use utils::process_chats;

pub fn routes() -> Router {
    Router::new()
        .typed_get(conversation::conversation)
        .typed_get(generated_output_canvas::generated_output_canvas)
        .typed_get(generated_output_download::generated_output_download)
        .typed_get(index::index)
        .typed_post(send_message::send_message)
        .typed_post(update_response::update_response)
        .typed_post(delete::delete)
        .typed_post(set_default_prompt::set_default_prompt)
        .layer(DefaultBodyLimit::max(50000000)) // 50MB limit for file uploads
}
