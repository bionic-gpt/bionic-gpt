#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineProps {
    condensed: Option<bool>,
    class: Option<String>,
    children: Element,
}

pub fn TimeLine(props: TimeLineProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let the_class = format!("timeline-item {}", class);

    let class = if props.condensed.is_some() {
        format!("timeline-condensed {}", the_class)
    } else {
        the_class
    };

    rsx!(
        div {
            class: "{class}",
            {{props.children}}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineBadgeProps {
    image_src: String,
    class: Option<String>,
}

pub fn TimeLineBadge(props: TimeLineBadgeProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("timeline-badge {}", class);
    rsx!(
        div {
            class: "{class}",
            img {
                src: "{props.image_src}",
                width: "16"
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TimeLineBodyProps {
    children: Element,
    class: Option<String>,
}

pub fn TimeLineBody(props: TimeLineBodyProps) -> Element {
    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("timeline-body {}", class);

    rsx!(
        div {
            class: "{class}",
            {props.children}
        }
    )
}
