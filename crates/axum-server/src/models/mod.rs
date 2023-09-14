mod form;
mod index;
use axum::{
    routing::{get, post},
    Router,
};

use ui_components::routes::models::{EDIT, INDEX, NEW};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, get(form::new))
        .route(EDIT, get(form::edit))
        .route(NEW, post(form::upsert))
}
