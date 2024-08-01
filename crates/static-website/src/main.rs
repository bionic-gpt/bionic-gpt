pub mod blog;
pub mod footer;
pub mod image_hero;
pub mod layout;
pub mod marketing;
pub mod navigation;
pub mod static_files;

use axum::Router;
use axum_extra::routing::RouterExt;
use dioxus::prelude::{ComponentFunction, Element, VirtualDom};
use std::net::SocketAddr;

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
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // build our application with a route
    let app = Router::new()
        .typed_get(static_files::static_path)
        .merge(blog::routes())
        .merge(marketing::routes());

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
