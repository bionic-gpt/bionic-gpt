mod delete;
mod index;
mod processing;
mod upload_doc;
use axum::{extract::DefaultBodyLimit, Router};
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_post(upload_doc::upload)
        .typed_post(delete::delete)
        .typed_get(processing::row)
        .layer(DefaultBodyLimit::max(50000000))
        .typed_get(index::index)
        .typed_get(index::index)
}
