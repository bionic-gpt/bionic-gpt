mod conversation;
mod delete;
mod delete_conv;
mod form;
mod image;
mod index;
mod new_chat;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_get(conversation::conversation)
        .typed_post(form::upsert)
        .typed_post(delete_conv::delete)
        .typed_post(delete::delete)
        .typed_get(image::image)
        .typed_get(new_chat::new_chat)
}
