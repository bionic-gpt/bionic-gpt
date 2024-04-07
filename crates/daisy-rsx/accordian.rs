#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AccordianProps {
    name: String,
    title: String,
    checked: Option<bool>,
    children: Element,
}

pub fn Accordian(props: AccordianProps) -> Element {
    rsx!(
        div {
            class: "collapse collapse-arrow bg-base-200",
            input {
                checked: props.checked,
                "type": "radio",
                name: props.name
            }
            div {
                class: "collapse-title text-md font-medium",
                "{props.title}"
            }
            div {
                class: "collapse-content  bg-base-200",
                {{props.children}}
            }
        }
    )
}
