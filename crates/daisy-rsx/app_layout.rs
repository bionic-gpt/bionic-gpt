#![allow(non_snake_case)]

use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props, Clone, PartialEq)]
pub struct AppLayoutProps {
    title: String,
    fav_icon_src: String,
    collapse_svg_src: String,
    stylesheets: Vec<String>,
    section_class: String,
    js_href: String,
    header: Element,
    children: Element,
    sidebar: Element,
    sidebar_footer: Element,
    sidebar_header: Element,
}

pub fn AppLayout(props: AppLayoutProps) -> Element {
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
            for href in &props.stylesheets {
                link {
                    rel: "stylesheet",
                    href: "{href}",
                    "type": "text/css"
                }
            }
            script {
                "type": "module",
                src: "{props.js_href}"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{props.fav_icon_src}"
            }
        }
        body {
            input {
                "type": "checkbox",
                id: "nav-toggle"
            }
            div {
                class: "l_layout",
                nav {
                    class: "l_navigation",
                    label {
                        id: "collapse-button",
                        "for": "nav-toggle",
                        img {
                            src: props.collapse_svg_src
                        }
                    }
                    div {
                        class: "l_nav_header flex items-center",
                        {props.sidebar_header}
                    }
                    div {
                        class: "l_nav_items",
                        {props.sidebar}
                    }
                    div {
                        class: "l_footer",
                        {props.sidebar_footer}
                    }
                }
                turbo-frame {
                    id: "main-content",
                    "data-turbo-action": "advance",
                    class: "l_content",
                    header {
                        label {
                            class: "hamburger",
                            "for": "nav-toggle",
                            div {
                                class: "top_bun"
                            }
                            div {
                                class: "meat"
                            }
                            div {
                                class: "bottom_bun"
                            }
                        }
                        div {
                            {props.header}
                        }
                    }
                    section {
                        class: props.section_class,
                        {props.children}
                    }
                }
            }
        }
    )
}
