#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BoxProps {
    class: Option<String>,
    children: Element,
}

pub fn Box(props: BoxProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("card {}", class);

    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct BoxHeadersProps {
    class: Option<String>,
    title: String,
    children: Element,
}

pub fn BoxHeader(props: BoxHeadersProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("card-header flex items-center {}", class);

    rsx!(
        div {
            class: "{class}",
            h3 {
                class: "card-title overflow-hidden",
                "{props.title}"
            }
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct BoxBodyProps {
    children: Element,
}

pub fn BoxBody(props: BoxBodyProps) -> Element {
    rsx!(
        div {
            class: "card-body",
            {props.children}
        }
    )
}
