mod delete;
mod form;
mod index;
use axum::{
    routing::{get, post},
    Router,
};

use web_pages::routes::models::{DELETE, INDEX, NEW};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(form::upsert))
        .route(DELETE, post(delete::delete))
}
