#![allow(non_snake_case)]
mod blog;
mod footer;
mod image_hero;
mod navigation;

use crate::blog::Blog;
use crate::footer::Footer;
use crate::image_hero::ImageHero;
use crate::navigation::Navigation;
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
        Navigation {

        }
        ImageHero {

        }
        Footer {

        }
    }
}
