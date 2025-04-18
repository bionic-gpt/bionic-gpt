pub mod api_keys;
pub mod api_pipeline;
pub mod audit_trail;
pub mod config;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod email;
pub mod errors;
pub mod history;
pub mod integrations;
pub mod jwt;
pub mod layout;
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

use axum_extra::routing::RouterExt;
pub use errors::CustomError;
pub use jwt::Jwt;

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

    // build our application with a route
    let app = Router::new()
        .typed_get(static_files::static_path)
        .typed_get(metrics::track_metrics)
        .typed_get(oidc_endpoint::index)
        .merge(api_pipeline::routes(&config))
        .merge(api_keys::routes())
        .merge(audit_trail::routes())
        .merge(console::routes())
        .merge(datasets::routes())
        .merge(documents::routes())
        .merge(history::routes())
        .merge(integrations::routes())
        .merge(llm_proxy::routes())
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
