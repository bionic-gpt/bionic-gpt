mod api_keys;
mod api_pipeline;
mod api_reverse_proxy;
mod audit_trail;
mod authentication;
mod config;
mod console;
mod datasets;
mod documents;
mod email;
mod errors;
mod index;
mod layout;
mod models;
mod pipelines;
mod profile;
mod prompt;
mod prompts;
mod registration_handler;
mod rls;
mod static_files;
mod team;
mod ui_completions;

use axum::extract::Extension;
use axum::routing::post;
use axum::{response::Html, routing::get};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::Config::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let config = config::Config::new();
    let pool = db::create_pool(&config.app_database_url);

    let axum_make_service = axum::Router::new()
        .route("/static/*path", get(static_files::static_path))
        .merge(api_pipeline::routes(&config))
        .route("/v1/*path", get(api_reverse_proxy::handler))
        .route("/v1/*path", post(api_reverse_proxy::handler))
        .route("/completions/:chat_id", post(ui_completions::handler))
        .route("/", get(index::index))
        .merge(team::routes())
        .merge(audit_trail::routes())
        .merge(profile::routes())
        .merge(registration_handler::routes())
        .merge(console::routes())
        .merge(api_keys::routes())
        .merge(datasets::routes())
        .merge(documents::routes())
        .merge(pipelines::routes())
        .merge(models::routes())
        .merge(prompts::routes())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .into_make_service();

    tracing::info!("listening on {}", addr);
    let server = hyper::Server::bind(&addr).serve(axum_make_service);

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}

pub fn render<F>(f: F) -> Html<&'static str>
where
    F: FnOnce(&mut Vec<u8>) -> Result<(), std::io::Error>,
{
    let mut buf = Vec::new();
    f(&mut buf).expect("Error rendering template");
    let html: String = String::from_utf8_lossy(&buf).into();

    Html(Box::leak(html.into_boxed_str()))
}
