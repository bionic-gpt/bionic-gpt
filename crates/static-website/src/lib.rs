pub mod architect_course_summary;
pub mod blog_summary;
pub mod components;
pub mod docs_summary;
pub mod generator;
pub mod layouts;
pub mod markdown;
pub mod pages;
pub mod pages_summary;

pub mod routes {
    pub const SIGN_IN_UP: &str = "https://app.bionic-gpt.com";

    pub mod blog {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/blog/")]
        pub struct Index {}
    }

    pub mod product {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/chat/")]
        pub struct Chat {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/assistants/")]
        pub struct Assistants {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/integrations/")]
        pub struct Integrations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/automations/")]
        pub struct Automations {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/product/developers/")]
        pub struct Developers {}
    }

    pub mod solutions {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/education/")]
        pub struct Education {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/solutions/support/")]
        pub struct Support {}
    }

    pub mod marketing {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/")]
        pub struct Index {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/terms/")]
        pub struct Terms {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/privacy/")]
        pub struct Privacy {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/pricing/")]
        pub struct Pricing {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/contact/")]
        pub struct Contact {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/partners/")]
        pub struct PartnersPage {}

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/services/")]
        pub struct ServicesPage {}
    }

    pub mod docs {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/docs/")]
        pub struct Index {}
    }

    pub mod architect_course {
        use axum_extra::routing::TypedPath;
        use serde::Deserialize;

        #[derive(TypedPath, Deserialize)]
        #[typed_path("/architect-course/")]
        pub struct Index {}
    }
}

use dioxus::prelude::Element;

pub fn render(page: Element) -> String {
    let html = dioxus_ssr::render_element(page);
    format!("<!DOCTYPE html><html lang='en'>{}</html>", html)
}
