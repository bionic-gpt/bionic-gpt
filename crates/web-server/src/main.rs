pub mod api_keys;
pub mod api_pipeline;
pub mod audit_trail;
pub mod auth;
pub mod config;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod email;
pub mod errors;
pub mod layout;
pub mod llm_reverse_proxy;
pub mod models;
pub mod oidc_endpoint;
pub mod pipelines;
pub mod profile;
pub mod prompts;
pub mod static_files;
pub mod team;

pub use auth::Authentication;
use axum_extra::routing::RouterExt;
pub use errors::CustomError;

use axum::{Extension, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = config::Config::new();
    let pool = db::create_pool(&config.app_database_url);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // build our application with a route
    let app = Router::new()
        .typed_get(static_files::static_path)
        .typed_get(oidc_endpoint::index)
        .merge(api_pipeline::routes(&config))
        .merge(llm_reverse_proxy::routes())
        .merge(team::routes())
        .merge(audit_trail::routes())
        .merge(profile::routes())
        .merge(console::routes())
        .merge(api_keys::routes())
        .merge(datasets::routes())
        .merge(documents::routes())
        .merge(pipelines::routes())
        .merge(models::routes())
        .merge(prompts::routes())
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
