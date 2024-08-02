use std::fs::File;
use std::io::Write;

use crate::image_hero::ImageHero;
use crate::layout::Layout;
use crate::routes::marketing::Index;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use dioxus::prelude::*;

pub fn routes() -> Router {
    Router::new().typed_get(index)
}

pub async fn generate() {
    let html = crate::render(HomePage).await;

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub async fn index(Index {}: Index) -> Html<String> {
    let html = crate::render(HomePage).await;

    Html(html)
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            title: "Test",
            ImageHero {}
        }
    }
}
