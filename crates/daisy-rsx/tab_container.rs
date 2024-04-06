#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TabContainerProps {
    class: Option<String>,
    children: Element,
}

pub fn TabContainer(props: TabContainerProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    rsx!(
        div {
            role: "tablist",
            class: "tabs tabs-bordered {class}",
            {{props.children}}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TabPanelProps {
    name: String,
    checked: Option<bool>,
    tab_name: String,
    children: Element,
}

pub fn TabPanel(props: TabPanelProps) -> Element {
    rsx!(
        input {
            checked: props.checked,
            "type": "radio",
            class: "tab",
            "aria-label": props.tab_name,
            name: props.name
        }
        div {
            role: "tabpanel",
            class: "tab-content",
            {{props.children}}
        }
    )
}
