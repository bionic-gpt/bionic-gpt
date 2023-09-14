#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct BoxProps<'a> {
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn Box<'a>(cx: Scope<'a, BoxProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("Box {}", class);

    cx.render(rsx!(
        div {
            class: "{class}",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct BoxHeadersProps<'a> {
    class: Option<&'a str>,
    title: &'a str,
    children: Element<'a>,
}

pub fn BoxHeader<'a>(cx: Scope<'a, BoxHeadersProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("Box-header d-flex flex-items-center {}", class);

    cx.render(rsx!(
        div {
            class: "{class}",
            h3 {
                class: "Box-title overflow-hidden",
                "{cx.props.title}"
            }
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct BoxBodyProps<'a> {
    children: Element<'a>,
}

pub fn BoxBody<'a>(cx: Scope<'a, BoxBodyProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "Box-body",
            &cx.props.children
        }
    ))
}
