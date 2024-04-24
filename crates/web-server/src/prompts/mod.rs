mod delete;
mod form;
mod index;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(form::upsert)
        .typed_post(delete::delete)
}
