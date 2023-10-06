mod form;
mod index;
use axum::{
    routing::{get, post},
    Router,
};

use ui_components::routes::models::{INDEX, NEW};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(form::upsert))
}
