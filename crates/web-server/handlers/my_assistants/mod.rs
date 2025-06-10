mod form;
mod loaders;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(loaders::my_assistants)
        .typed_get(loaders::view_assistant)
        .typed_get(loaders::manage_datasets)
        .typed_get(loaders::manage_integrations)
        // Actions
        .typed_post(form::upsert)
        .typed_post(form::update_datasets_action)
        .typed_post(form::update_integrations_action)
}
