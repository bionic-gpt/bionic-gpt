mod index;
mod upload_doc;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use ui_components::routes::documents::{INDEX, UPLOAD};

pub fn routes() -> Router {
    Router::new()
        .route(UPLOAD, post(upload_doc::upload))
        .layer(DefaultBodyLimit::max(50000000))
        .route(INDEX, get(index::index))
}
