#![allow(non_snake_case)]

use crate::components::navigation::{Navigation, Section};
use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    title: String,
    description: String,
    image: Option<String>,
    children: Element,
    mobile_menu: Option<Element>,
    section: Section,
}

pub fn Layout(props: LayoutProps) -> Element {
    let image = props.image.unwrap_or("/open-graph.png".to_string());
    rsx!(
        head {
            title {
                "{props.title}"
            }
            meta {
                charset: "utf-8"
            }
            meta {
                "http-equiv": "X-UA-Compatible",
                content: "IE=edge"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1"
            }
            meta {
                name: "description",
                content: "{props.description}"
            }

            // The four required Open Graph tags for every page are og:title, og:type, og:image, and og:url.
            meta { property: "og:title", content: "{props.title}" }
            meta { property: "og:description", content: "{props.description}" }
            meta { property: "og:type", content: "article" }
            meta { property: "og:site_name", content: "Bionic GPT" }
            meta { property: "og:image", content: "{image}" }
            meta { property: "twitter:image", content: "{image}" }

            link {
                rel: "stylesheet",
                href: "/tailwind.css",
                "type": "text/css"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "/favicon.svg"
            }
            script {
                "async": "true",
                "data-goatcounter": "https://bionicgpt.goatcounter.com/count",
                src: "/goat-counter.js"

            }
            script {
                "async": "true",
                src: "/copy-paste.js"

            }
            script {
                "type": "module",
                src: "https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"
            }
        }
        body {
            //WebinarHeader {}
            Navigation {
                mobile_menu: props.mobile_menu,
                section: props.section
            }
            {props.children}
            script {
                src: "https://instant.page/5.2.0",
                type: "module",
                integrity: "sha384-jnZyxPjiipYXnSU0ygqeac2q7CVYMbh84q0uHVRRxEtvFPiQYbXWUorga2aqZJ0z"
            }
        }
    )
}
