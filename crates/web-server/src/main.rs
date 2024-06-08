pub mod api_keys;
pub mod api_pipeline;
pub mod audit_trail;
pub mod auth;
pub mod barricade_endpoint;
pub mod config;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod email;
pub mod errors;
pub mod guardrails;
pub mod layout;
pub mod licence;
pub mod llm_reverse_proxy;
pub mod metrics;
pub mod models;
pub mod oidc_endpoint;
pub mod pipelines;
pub mod profile;
pub mod prompts;
pub mod rate_limits;
pub mod static_files;
pub mod team;
pub mod teams;

pub use auth::Authentication;
use axum_extra::routing::RouterExt;
pub use errors::CustomError;

use axum::{Extension, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::new();
    let pool = db::create_pool(&config.app_database_url);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Set up post registration/sign in endpoints
    // based on whether we use oauth2 proxy or barricade
    let auth_routes = if config.enable_barricade {
        Router::new()
            .typed_get(barricade_endpoint::index)
            .typed_get(barricade_endpoint::post_registration)
    } else {
        Router::new().typed_get(oidc_endpoint::index)
    };

    // build our application with a route
    let app = Router::new()
        .typed_get(static_files::static_path)
        .typed_get(metrics::track_metrics)
        .merge(auth_routes)
        .merge(api_pipeline::routes(&config))
        .merge(api_keys::routes())
        .merge(audit_trail::routes())
        .merge(console::routes())
        .merge(datasets::routes())
        .merge(documents::routes())
        .merge(guardrails::routes())
        .merge(licence::routes())
        .merge(llm_reverse_proxy::routes())
        .merge(models::routes())
        .merge(pipelines::routes())
        .merge(profile::routes())
        .merge(prompts::routes())
        .merge(rate_limits::routes())
        .merge(team::routes())
        .merge(teams::routes())
        .layer(Extension(config.clone()))
        .layer(Extension(pool.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
