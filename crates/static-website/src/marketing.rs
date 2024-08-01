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
