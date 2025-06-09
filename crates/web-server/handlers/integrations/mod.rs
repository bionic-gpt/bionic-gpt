use axum::Router;
use axum_extra::routing::RouterExt;

// Module declarations
pub mod actions;
pub mod configuration_actions;
pub mod helpers;
pub mod loaders;

// Re-export all public functions for backward compatibility
pub use actions::{delete_action, edit_action, new_action};
pub use configuration_actions::{
    configure_api_key_action, delete_api_key_connection_action, ApiKeyForm,
};
pub use helpers::parse_openapi_spec;
pub use loaders::{edit_loader, loader, new_loader, view_loader};

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(loader)
        .typed_get(view_loader)
        .typed_get(new_loader)
        .typed_get(edit_loader)
        // Actions
        .typed_post(new_action)
        .typed_post(edit_action)
        .typed_post(delete_action)
        .typed_post(configure_api_key_action)
        .typed_post(delete_api_key_connection_action)
}
