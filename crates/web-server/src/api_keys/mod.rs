pub mod delete;
pub mod index;
pub mod new;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(new::new_api_key)
        .typed_post(delete::delete)
}
