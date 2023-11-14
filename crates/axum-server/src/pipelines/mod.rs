mod index;

use axum::{routing::get, Router};

use ui_components::routes::documents::BULK;

pub fn routes() -> Router {
    Router::new().route(BULK, get(index::index))
}
