#![allow(non_snake_case)]

use crate::components::navigation::Section;
use crate::routes::{blog, docs, marketing};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub children: Element,
    pub mobile_menu: Option<Element>,
    pub section: Section,
}

fn nav_link(current: Section, target: Section, href: &str, label: &str) -> Element {
    let class = if current == target {
        "font-semibold"
    } else {
        ""
    };

    rsx!(
        li {
            a { class: "{class}", href: "{href}", "{label}" }
        }
    )
}

pub fn Layout(mut props: LayoutProps) -> Element {
    let links = vec![
        (Section::Home, marketing::Index {}.to_string(), "Home"),
        (
            Section::Pricing,
            marketing::Pricing {}.to_string(),
            "Pricing",
        ),
        (
            Section::Partners,
            marketing::PartnersPage {}.to_string(),
            "Partners",
        ),
        (
            Section::McpServers,
            marketing::McpServers {}.to_string(),
            "MCP Servers",
        ),
        (Section::Blog, blog::Index {}.to_string(), "Blog"),
        (Section::Docs, docs::Index {}.to_string(), "Docs"),
        (
            Section::Contact,
            marketing::Contact {}.to_string(),
            "Contact",
        ),
    ];

    let mobile_menu = props.mobile_menu.take();
    let image = props
        .image
        .clone()
        .unwrap_or_else(|| "/open-graph.png".to_string());

    rsx!(
        head {
            title { "{props.title}" }
            meta { charset: "utf-8" }
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
            meta { property: "og:title", content: "{props.title}" }
            meta { property: "og:description", content: "{props.description}" }
            meta { property: "og:type", content: "article" }
            meta { property: "og:site_name", content: "Deploy" }
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
                "data-goatcounter": "https://deploy.goatcounter.com/count",
                src: "/goat-counter.js",
            }
            script {
                "async": "true",
                src: "/copy-paste.js",
            }
            script {
                "type": "module",
                src: "https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"
            }
        }
        body {
            class: "min-h-screen bg-base-100 text-base-content",
            header {
                class: "px-6 py-6 flex flex-col gap-4 md:flex-row md:items-center md:justify-between",
                a { class: "text-2xl font-bold", href: marketing::Index {}.to_string(), "Deploy" }
                ul {
                    class: "flex flex-wrap gap-4",
                    for (target, href, label) in links {
                        {nav_link(props.section.clone(), target, &href, label)}
                    }
                }
                a {
                    class: "btn btn-primary",
                    href: marketing::Contact {}.to_string(),
                    "Talk to us"
                }
            }
            if let Some(menu) = mobile_menu {
                nav {
                    class: "px-6 md:hidden",
                    {menu}
                }
            }
            main { class: "", {props.children} }
        }
    )
}
