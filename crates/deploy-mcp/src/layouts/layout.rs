#![allow(non_snake_case)]

use crate::components::navigation::Section;
use crate::routes::{blog, docs, marketing};
use dioxus::prelude::*;

const BASE_URL: &str = "https://deploy-mcp.com";

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub url: Option<String>,
    pub children: Element,
    pub mobile_menu: Option<Element>,
    pub section: Section,
}

fn absolute_url(value: &str) -> String {
    if value.starts_with("http://") || value.starts_with("https://") {
        value.to_string()
    } else if value.starts_with('/') {
        format!("{BASE_URL}{value}")
    } else {
        let trimmed = value.trim_start_matches('/');
        format!("{BASE_URL}/{trimmed}")
    }
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
            Section::Enterprise,
            marketing::Enterprise {}.to_string(),
            "Enterprise",
        ),
        (
            Section::Pricing,
            marketing::Pricing {}.to_string(),
            "Pricing",
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
        (
            Section::None,
            crate::routes::SIGN_IN_UP.to_string(),
            "Login",
        ),
    ];

    let mobile_menu = props.mobile_menu.take();
    let image_path = props
        .image
        .clone()
        .unwrap_or_else(|| "/open-graph.png".to_string());
    let image_meta = absolute_url(&image_path);
    let page_url = props.url.clone().unwrap_or_else(|| BASE_URL.to_string());
    let page_url = absolute_url(&page_url);

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
            meta { property: "og:url", content: "{page_url}" }
            meta { property: "og:image", content: "{image_meta}" }
            meta { name: "twitter:card", content: "summary_large_image" }
            meta { name: "twitter:title", content: "{props.title}" }
            meta { name: "twitter:description", content: "{props.description}" }
            meta { property: "twitter:image", content: "{image_meta}" }
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
                    href: crate::routes::SIGN_IN_UP,
                    "Get started"
                }
            }
            if let Some(menu) = mobile_menu {
                nav {
                    class: "px-6 md:hidden",
                    {menu}
                }
            }
            main { class: "", {props.children} }
            script {
                src: "https://instant.page/5.2.0",
                type: "module",
                integrity: "sha384-jnZyxPjiipYXnSU0ygqeac2q7CVYMbh84q0uHVRRxEtvFPiQYbXWUorga2aqZJ0z"
            }
        }
    )
}
