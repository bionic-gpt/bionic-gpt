mod actions;
mod loader;

pub use actions::*;
pub use loader::*;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader::loader)
        .typed_get(loader::new_loader)
        .typed_post(actions::action_create)
        .typed_post(actions::action_delete)
}
