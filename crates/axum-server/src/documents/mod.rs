mod delete;
mod index;
mod status;
mod upload_doc;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use ui_pages::routes::documents::{DELETE, INDEX, STATUS, UPLOAD};

pub fn routes() -> Router {
    Router::new()
        .route(UPLOAD, post(upload_doc::upload))
        .route(DELETE, post(delete::delete))
        .layer(DefaultBodyLimit::max(50000000))
        .route(INDEX, get(index::index))
        .route(STATUS, get(status::status))
}
