mod filter;
mod index;

pub const PAGE_SIZE: i64 = 10;

use axum::{
    routing::{get, post},
    Router,
};

use ui_pages::routes::audit_trail::INDEX;

pub fn routes() -> Router {
    Router::new()
        .route(INDEX, get(index::index))
        .route(INDEX, post(filter::filter))
}
