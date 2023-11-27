#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct TabContainerProps<'a> {
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn TabContainer<'a>(cx: Scope<'a, TabContainerProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    cx.render(rsx!(
        div {
            role: "tablist",
            class: "tabs tabs-bordered {class}",
            {&cx.props.children}
        }
    ))
}

#[derive(Props)]
pub struct TabPanelProps<'a> {
    name: &'a str,
    checked: Option<bool>,
    tab_name: &'a str,
    children: Element<'a>,
}

pub fn TabPanel<'a>(cx: Scope<'a, TabPanelProps<'a>>) -> Element {
    cx.render(rsx!(
        input {
            checked: cx.props.checked,
            "type": "radio",
            class: "tab",
            "aria-label": cx.props.tab_name,
            name: cx.props.name
        }
        div {
            role: "tabpanel",
            class: "tab-content",
            {&cx.props.children}
        }
    ))
}
