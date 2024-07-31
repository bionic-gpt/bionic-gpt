//! Run with:
//!
//! ```sh
//! dx serve --platform fullstack
//! ```
#![allow(non_snake_case, unused)]
mod blog;
mod footer;
mod image_hero;
mod navigation;

use crate::blog::Blog;
use crate::footer::Footer;
use crate::image_hero::ImageHero;
use crate::navigation::Navigation;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:slug/")]
    Blog { slug: String },
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Navigation {

        }
        ImageHero {

        }
        Footer {

        }
    }
}

#[server]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);

    Ok(())
}

#[server]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello".to_string())
}

fn main() {
    #[cfg(feature = "web")]
    tracing_wasm::set_as_global_default();

    //#[cfg(feature = "server")]
    //tracing_subscriber::fmt::init();

    launch(App);
}
