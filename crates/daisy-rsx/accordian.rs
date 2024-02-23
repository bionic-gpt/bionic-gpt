#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct AccordianProps<'a> {
    name: &'a str,
    title: &'a str,
    checked: Option<bool>,
    children: Element<'a>,
}

pub fn Accordian<'a>(cx: Scope<'a, AccordianProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "collapse collapse-arrow bg-base-200",
            input {
                checked: cx.props.checked,
                "type": "radio",
                name: cx.props.name
            }
            div {
                class: "collapse-title text-md font-medium",
                "{cx.props.title}"
            }
            div {
                class: "collapse-content  bg-base-200",
                {&cx.props.children}
            }
        }
    ))
}
