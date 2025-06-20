mod loader;
mod actions;

pub use loader::*;
pub use actions::*;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader::loader)
        .typed_post(actions::action_upsert)
        .typed_post(actions::action_delete)
}
