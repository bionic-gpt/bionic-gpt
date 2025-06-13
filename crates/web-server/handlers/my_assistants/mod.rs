mod assistant_actions;
mod assistant_loaders;
mod datasets;
mod integrations;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(assistant_loaders::my_assistants)
        .typed_get(datasets::manage_datasets)
        .typed_get(integrations::manage_integrations)
        // Actions
        .typed_post(assistant_actions::upsert)
        .typed_post(datasets::update_datasets_action)
        .typed_post(integrations::add_integration_action)
        .typed_post(integrations::remove_integration_action)
}
