mod form;
mod loaders;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(loaders::my_assistants)
        // Actions
        .typed_post(form::upsert)
}
