#![allow(non_snake_case)]
mod blog;
mod navigation;

use crate::blog::Blog;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:slug")]
    Blog { slug: String },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Link {
            to: Route::Blog {
                slug: "banning-chat-gpt".to_string()
            },
            "Go to blog"
        }
    }
}
