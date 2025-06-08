#![allow(non_snake_case)]

use assets::files::collapse_svg;
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

pub fn BaseLayout(props: AppLayoutProps) -> Element {
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
            div {
                class: "flex h-screen bg-base-300",
                nav {
                    id: "sidebar",
                    class: "
                        fixed
                        bg-base-300
                        inset-y-0
                        left-0
                        w-64
                        transform
                        -translate-x-full
                        transition-transform
                        duration-200
                        ease-in-out
                        flex
                        flex-col
                        lg:translate-x-0
                        lg:static
                        lg:inset-auto
                        lg:transform-none
                        z-20",
                    div {
                        class: "flex items-center pl-4 pr-2 pt-4",
                        {props.sidebar_header}
                    }
                    div {
                        class: "flex-1 overflow-y-auto",
                        {props.sidebar}
                    }
                    div {
                        class: "pb-4 pl-4 pr-2",
                        {props.sidebar_footer}
                    }
                }
                turbo-frame {
                    id: "main-content",
                    class: "flex-1 flex flex-col bg-base-200 m-2 border border-base-300 shadow  overflow-x-hidden",
                    header {
                        class: "flex items-center p-4",
                        button {
                            id: "toggleButton",
                            img {
                                height: "24",
                                width: "24",
                                class: "svg-icon mr-6",
                                src: collapse_svg.name
                            }
                        }
                        div {
                            class: "flex items-center w-full justify-between",
                            {props.header}
                        }
                    }
                    section {
                        class: "{props.section_class} flex-1 overflow-y-auto",
                        {props.children}
                    }
                }
            }
        }
    )
}
