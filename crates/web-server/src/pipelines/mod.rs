mod delete;
mod index;
mod new;

use axum::{
    routing::{get, post},
    Router,
};

use web_pages::routes::document_pipelines::{DELETE, INDEX, NEW};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new::new))
        .route(DELETE, post(delete::delete))
}
