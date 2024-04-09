#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::{Router, Extension};
    use leptos::*;
    use web_ui::fileserv::file_and_error_handler;
    use web_ui::pages;
    use web_ui::ssr;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let config = ssr::config::Config::new();
    let pool = db::create_pool(&config.app_database_url);

    // build our application with a route
    let app = Router::new()
        // Routes implemented with Leptos
        .route("/leptos_api_keys", get(pages::api_keys::index))
        .route("/leptos_console", get(pages::console::index))
        // Original Dioxus routes
        .route("/app/team/:team_id/api_keys", get(ssr::api_keys::index::index))
        //.merge(ssr::api_keys::routes())
        .fallback(file_and_error_handler)
        .layer(Extension(leptos_options.clone()))
        .with_state(leptos_options)
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
