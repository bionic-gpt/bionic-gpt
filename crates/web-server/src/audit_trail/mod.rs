mod filter;
mod index;

pub const PAGE_SIZE: i64 = 10;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(filter::filter)
}
