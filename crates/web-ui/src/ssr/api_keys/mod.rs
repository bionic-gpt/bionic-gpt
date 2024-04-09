pub mod delete;
pub mod index;
pub mod new;

use axum::{
    routing::{get, post},
    Router,
};

use ui_pages::routes::api_keys::{DELETE, INDEX, NEW};

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(NEW, post(new::new_api_key))
        .route(DELETE, post(delete::delete))
}
