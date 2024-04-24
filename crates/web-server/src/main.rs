pub mod api_keys;
pub mod api_pipeline;
pub mod api_reverse_proxy;
pub mod audit_trail;
pub mod auth;
pub mod config;
pub mod console;
pub mod datasets;
pub mod documents;
pub mod email;
pub mod errors;
pub mod layout;
pub mod models;
pub mod oidc_endpoint;
pub mod pipelines;
pub mod profile;
pub mod prompt;
pub mod prompts;
pub mod static_files;
pub mod team;
pub mod ui_completions;

pub use auth::Authentication;
pub use errors::CustomError;

use axum::routing::{get, post};
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
        .route("/static/*path", get(static_files::static_path))
        .route("/", get(oidc_endpoint::index))
        .merge(api_pipeline::routes(&config))
        .route("/v1/*path", get(api_reverse_proxy::handler))
        .route("/v1/*path", post(api_reverse_proxy::handler))
        .route("/completions/:chat_id", post(ui_completions::handler))
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
