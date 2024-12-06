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
    mobile_menu: Element,
    section: Section,
}

pub fn Layout(props: LayoutProps) -> Element {
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
            meta {
                "property": "og:description",
                content: "{props.description}"
            }
            meta {
                "property": "og:title",
                content: "{props.title}"
            }
            if let Some(image) = props.image {
                {rsx!(
                    meta {
                        "property": "og:image",
                        content: "{image}"
                    }
                )}
            }
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
                src: "https://unpkg.com/htmx.org@2.0.3",
                integrity: "sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq",
                crossorigin: "anonymous"
            }
        }
        body {
            //WebinarHeader {}
            Navigation {
                mobile_menu: props.mobile_menu,
                section: props.section
            }
            {props.children}
        }
    )
}
