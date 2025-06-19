mod actions;
mod delete;
mod index;
mod loaders;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(index::loader)
        .typed_get(loaders::new_automation_loader)
        .typed_get(loaders::edit_automation_loader)
        // Actions
        .typed_post(actions::upsert)
        .typed_post(delete::delete)
}
