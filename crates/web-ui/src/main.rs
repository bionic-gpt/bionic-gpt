#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use axum::{Extension, Router};
    use leptos::*;
    use web_ui::fileserv;
    use web_ui::pages;
    use web_ui::ssr;

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
        .route("/static/*path", get(ssr::static_files::static_path))
        .route("/", get(ssr::oidc_endpoint::index))
        .merge(ssr::api_pipeline::routes(&config))
        //.route("/v1/*path", get(api_reverse_proxy::handler))
        //.route("/v1/*path", post(api_reverse_proxy::handler))
        //.route("/completions/:chat_id", post(ui_completions::handler))
        //.merge(ssr::team::routes())
        .merge(ssr::audit_trail::routes())
        .merge(ssr::profile::routes())
        .merge(ssr::console::routes())
        .merge(ssr::api_keys::routes())
        .merge(ssr::datasets::routes())
        .merge(ssr::documents::routes())
        .merge(ssr::pipelines::routes())
        .merge(ssr::models::routes())
        .merge(ssr::prompts::routes())
        .fallback(fileserv::file_and_error_handler)
        .layer(Extension(leptos_options.clone()))
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
}
