#![allow(non_snake_case)]
#![allow(unused_braces)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct TimeLineProps<'a> {
    condensed: Option<bool>,
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn TimeLine<'a>(cx: Scope<'a, TimeLineProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let the_class = format!("timeline-item {}", class);

    let class = if cx.props.condensed.is_some() {
        format!("timeline-condensed {}", the_class)
    } else {
        the_class
    };

    cx.render(rsx!(
        div {
            class: "{class}",
            {&cx.props.children}
        }
    ))
}

#[derive(Props)]
pub struct TimeLineBadgeProps<'a> {
    image_src: &'a str,
    class: Option<&'a str>,
}

pub fn TimeLineBadge<'a>(cx: Scope<'a, TimeLineBadgeProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("timeline-badge {}", class);
    cx.render(rsx!(
        div {
            class: "{class}",
            img {
                src: "{cx.props.image_src}",
                width: "16"
            }
        }
    ))
}

#[derive(Props)]
pub struct TimeLineBodyProps<'a> {
    children: Element<'a>,
}

pub fn TimeLineBody<'a>(cx: Scope<'a, TimeLineBodyProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "timeline-body",
            &cx.props.children
        }
    ))
}
