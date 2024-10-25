mod conversation;
mod delete;
mod index;
mod send_message;
mod update_response;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(conversation::conversation)
        .typed_get(index::index)
        .typed_post(send_message::send_message)
        .typed_post(update_response::update_response)
        .typed_post(delete::delete)
}
