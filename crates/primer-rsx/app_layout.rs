#![allow(non_snake_case)]

use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props)]
pub struct AppLayoutProps<'a> {
    title: &'a str,
    fav_icon_src: &'a str,
    collapse_svg_src: &'a str,
    stylesheets: Vec<String>,
    section_class: &'a str,
    js_href: &'a str,
    header: Element<'a>,
    children: Element<'a>,
    sidebar: Element<'a>,
    sidebar_footer: Element<'a>,
    sidebar_header: Element<'a>,
}

pub fn AppLayout<'a>(cx: Scope<'a, AppLayoutProps<'a>>) -> Element {
    cx.render(rsx!(
        head {
            title {
                "{cx.props.title}"
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
            for href in &cx.props.stylesheets {
                cx.render(rsx!(
                    link {
                        rel: "stylesheet",
                        href: "{href}",
                        "type": "text/css"
                    }
                ))
            }
            script {
                "type": "module",
                src: "{cx.props.js_href}"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{cx.props.fav_icon_src}"
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
                            src: cx.props.collapse_svg_src
                        }
                    }
                    div {
                        class: "l_nav_header flex items-center",
                        &cx.props.sidebar_header
                    }
                    div {
                        class: "l_nav_items",
                        &cx.props.sidebar
                    }
                    div {
                        class: "l_footer",
                        &cx.props.sidebar_footer
                    }
                }
                turbo-frame {
                    id: "main-content",
                    "data-turbo-action": "advance",
                    target: "_top",
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
                        &cx.props.header
                    }
                    section {
                        class: cx.props.section_class,
                        &cx.props.children
                    }
                }
            }
        }
    ))
}
