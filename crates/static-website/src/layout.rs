#![allow(non_snake_case)]

use crate::footer::Footer;
use crate::navigation::Navigation;
use crate::templates::statics::favicon_ico;
use dioxus::prelude::*;

// Remember: owned props must implement PartialEq!
#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    title: String,
    children: Element,
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
            link {
                rel: "stylesheet",
                href: "/tailwind.css",
                "type": "text/css"
            }
            link {
                rel: "icon",
                "type": "image/svg+xml",
                href: "{favicon_ico.name}"
            }
        }
        body {
            Navigation {}
            {props.children}
            Footer {}
        }
    )
}
