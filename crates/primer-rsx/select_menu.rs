#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectMenuAlignment {
    Default,
    Right,
}

impl SelectMenuAlignment {
    pub fn to_string(&self) -> &'static str {
        match self {
            SelectMenuAlignment::Default => "SelectMenu",
            SelectMenuAlignment::Right => "SelectMenu right-0",
        }
    }
}

#[derive(Props)]
pub struct SelectMenuProps<'a> {
    alignment: Option<SelectMenuAlignment>,
    summary: Element<'a>,
    children: Element<'a>,
}

pub fn SelectMenu<'a>(cx: Scope<'a, SelectMenuProps<'a>>) -> Element {
    let class = if cx.props.alignment.is_some() {
        cx.props.alignment.unwrap().to_string()
    } else {
        SelectMenuAlignment::Default.to_string()
    };

    if cx.props.alignment.is_some() && cx.props.alignment.unwrap() == SelectMenuAlignment::Right {
        cx.render(rsx!(
            div {
                class: "d-flex flex-justify-end position-relative",
                details {
                    class: "details-reset details-overlay",
                    &cx.props.summary,
                    div {
                        class: "{class}",
                        &cx.props.children
                    }
                }
            }
        ))
    } else {
        cx.render(rsx!(
            details {
                class: "details-reset details-overlay position-relative",
                &cx.props.summary,
                div {
                    class: "{class}",
                    &cx.props.children
                }
            }
        ))
    }
}

#[derive(Props)]
pub struct SelectMenuModalProps<'a> {
    children: Element<'a>,
}

pub fn SelectMenuModal<'a>(cx: Scope<'a, SelectMenuModalProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "SelectMenu-modal",
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct SelectMenuListProps<'a> {
    children: Element<'a>,
}

pub fn SelectMenuList<'a>(cx: Scope<'a, SelectMenuListProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "SelectMenu-list",
            &cx.props.children
        }
    ))
}
