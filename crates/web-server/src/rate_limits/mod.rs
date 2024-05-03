pub mod delete;
pub mod index;
pub mod upsert;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(upsert::upsert)
        .typed_post(delete::delete)
}
