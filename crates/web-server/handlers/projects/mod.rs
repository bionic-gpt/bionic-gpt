pub mod actions;
pub mod loader;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader::index)
        .typed_get(loader::view)
        .typed_post(actions::action_upsert)
        .typed_post(actions::action_delete)
        .typed_post(actions::action_start_chat)
}
