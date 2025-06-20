mod actions;
mod delete;
mod index;
mod integrations;
mod loaders;
mod triggers;

use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new()
        // Loaders
        .typed_get(index::loader)
        .typed_get(loaders::new_automation_loader)
        .typed_get(loaders::edit_automation_loader)
        .typed_get(integrations::manage_integrations)
        .typed_get(triggers::manage_triggers)
        // Actions
        .typed_post(actions::upsert)
        .typed_post(integrations::add_integration_action)
        .typed_post(integrations::remove_integration_action)
        .typed_post(triggers::add_cron_trigger)
        .typed_post(triggers::remove_cron_trigger)
        .typed_post(delete::delete)
}
