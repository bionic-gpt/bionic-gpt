#[path = "crud/handlers.rs"]
pub mod crud;
#[path = "form/handlers.rs"]
pub mod form;
#[path = "loader/loader.rs"]
pub mod loader;
pub mod support;
pub mod ui;

use axum::Router;
use axum_extra::routing::RouterExt;
use std::sync::Arc;
use support::Store;

// Install real authentication middleware outside this router before serving it.
pub fn routes(store: Arc<Store>) -> Router {
    Router::new()
        .typed_get(loader::loader)
        .typed_get(form::new_loader)
        .typed_post(form::action_create)
        .typed_get(crud::edit_loader)
        .typed_post(crud::action_update)
        .typed_post(crud::action_delete)
        .with_state(store)
}
