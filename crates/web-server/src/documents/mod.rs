mod delete;
mod index;
mod upload_doc;
mod processing;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use web_pages::routes::documents::{DELETE, INDEX, UPLOAD, PROCESSING};

pub fn routes() -> Router {
    Router::new()
        .route(UPLOAD, post(upload_doc::upload))
        .route(DELETE, post(delete::delete))
        .route(PROCESSING, get(processing::row))
        .layer(DefaultBodyLimit::max(50000000))
        .route(INDEX, get(index::index))
}
