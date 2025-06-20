mod loader;
mod actions;

pub use loader::*;
pub use actions::*;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader::loader)
        .typed_post(actions::action_new_api_key)
        .typed_post(actions::action_delete_api_key)
}
