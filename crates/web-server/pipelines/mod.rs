mod delete;
mod index;
mod new;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(index::index)
        .typed_post(new::new)
        .typed_post(delete::delete)
}
