pub mod config;
pub mod email;
pub mod errors;
pub mod handlers;
pub mod jwt;
pub mod layout;

use axum_extra::routing::RouterExt;
pub use errors::CustomError;
pub use jwt::Jwt;

use axum::{Extension, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get log level from environment variable or default to INFO
    let log_level = std::env::var("LOG_LEVEL")
        .map(|level| match level.to_uppercase().as_str() {
            "TRACE" => tracing::Level::TRACE,
            "DEBUG" => tracing::Level::DEBUG,
            "INFO" => tracing::Level::INFO,
            "WARN" => tracing::Level::WARN,
            "ERROR" => tracing::Level::ERROR,
            _ => {
                eprintln!("Unknown log level: {}, defaulting to INFO", level);
                tracing::Level::INFO
            }
        })
        .unwrap_or(tracing::Level::INFO);

    // Create a filter that only enables your crates and disables others
    let filter = tracing_subscriber::EnvFilter::new("")
        // Disable all crates by default
        .add_directive(tracing_subscriber::filter::LevelFilter::OFF.into())
        // Enable your crates with the specified log level
        // Adjust these patterns to match your crate names
        .add_directive(format!("web_server={}", log_level).parse().unwrap())
        .add_directive(format!("db={}", log_level).parse().unwrap())
        .add_directive(format!("llm_proxy={}", log_level).parse().unwrap())
        .add_directive(format!("integrations={}", log_level).parse().unwrap())
        .add_directive(format!("embeddings_api={}", log_level).parse().unwrap())
        // Add more of your crates as needed
        ;

    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Set up panic hook with logging
    std::panic::set_hook(Box::new(|panic_info| {
        // Extract panic information
        let backtrace = std::backtrace::Backtrace::capture();

        let location = panic_info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => s.as_str(),
                None => "Unknown panic message",
            },
        };

        // Log the panic information
        tracing::error!(
            message = %message,
            location = %location,
            backtrace = %backtrace,
            "PANIC OCCURRED"
        );
    }));

    let config = config::Config::new();
    let pool = db::create_pool(&config.app_database_url);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // build our application with a route
    let app = Router::new()
        .typed_get(handlers::static_files::static_path)
        .typed_get(handlers::metrics::track_metrics)
        .typed_get(handlers::oidc_endpoint::index)
        .merge(handlers::api_pipeline::routes(&config))
        .merge(handlers::api_keys::routes())
        .merge(handlers::audit_trail::routes())
        .merge(handlers::console::routes())
        .merge(handlers::datasets::routes())
        .merge(handlers::documents::routes())
        .merge(handlers::history::routes())
        .merge(handlers::integrations::routes())
        .merge(handlers::oauth2::routes())
        .merge(llm_proxy::routes())
        .merge(handlers::models::routes())
        .merge(handlers::pipelines::routes())
        .merge(handlers::profile::routes())
        .merge(handlers::assistants::routes())
        .merge(handlers::rate_limits::routes())
        .merge(handlers::team::routes())
        .merge(handlers::teams::routes())
        .merge(handlers::workflows::routes())
        .layer(Extension(config.clone()))
        .layer(Extension(pool.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
