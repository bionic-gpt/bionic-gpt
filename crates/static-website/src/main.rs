pub mod blog;
pub mod blog_summary;
pub mod components;
pub mod docs;
pub mod docs_summary;
pub mod footer;
pub mod static_files;
pub mod summary;

use axum::Router;
use dioxus::prelude::{ComponentFunction, Element, VirtualDom};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

pub mod routes {

    pub mod blog {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/blog/:slug")]
        pub struct Index {
            pub slug: String,
        }
    }

    pub mod marketing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/")]
        pub struct Index {}
    }

    pub mod docs {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/docs/:section/:title")]
        pub struct Index {
            pub section: String,
            pub title: String,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    components::marketing::generate().await;
    summary::generate_blog_list(blog_summary::summary()).await;
    summary::generate(docs_summary::summary());
    summary::generate(blog_summary::summary());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // build our application with a route
    let app = Router::new().nest_service("/", ServeDir::new("dist"));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Generic function to render a component and its props to a string
pub fn render_with_props<P: Clone + 'static, M: 'static>(
    root: impl ComponentFunction<P, M>,
    root_props: P,
) -> String {
    let mut vdom = VirtualDom::new_with_props(root, root_props);
    vdom.rebuild_in_place();
    let html = dioxus_ssr::render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

async fn render(ele: fn() -> Element) -> String {
    // create a VirtualDom with the app component
    let mut vdom = VirtualDom::new(ele);
    // rebuild the VirtualDom before rendering
    vdom.rebuild_in_place();

    // render the VirtualDom to HTML
    let html = dioxus_ssr::render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
