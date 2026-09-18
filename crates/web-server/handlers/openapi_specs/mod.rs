mod actions;
mod loader;

pub use actions::*;
pub use loader::*;

use axum::{extract::DefaultBodyLimit, Router};
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        .typed_get(loader::index_loader)
        .typed_get(loader::new_loader)
        .typed_get(loader::edit_loader)
        .typed_post(actions::action_import)
        .typed_post(actions::action_upsert)
        .typed_post(actions::action_delete)
        .layer(DefaultBodyLimit::max(actions::MAX_UPLOAD_BYTES))
}
