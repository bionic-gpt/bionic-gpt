mod delete;
mod form;
mod index;
mod new_chat;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(form::upsert)
        .typed_post(delete::delete)
        .typed_get(new_chat::new_chat)
}
