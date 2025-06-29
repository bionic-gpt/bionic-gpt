#![allow(non_snake_case)]
use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    #[default]
    None,
    Top,
    Bottom,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::None => write!(f, ""),
            Direction::Top => write!(f, "dropdown-top"),
            Direction::Bottom => write!(f, "dropdown-bottom"),
            Direction::Left => write!(f, "dropdown-left"),
            Direction::Right => write!(f, "dropdown-right"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownProps {
    children: Element,
    carat: Option<bool>,
    button_text: String,
    class: Option<String>,
    direction: Option<Direction>,
    /// When set the dropdown trigger will render like a navigation link
    link_style: Option<bool>,
    /// When set the dropdown will open on hover
    hover: Option<bool>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
}

#[component]
pub fn DropDown(props: DropDownProps) -> Element {
    let direction = props.direction.unwrap_or_default();

    let hover_class = if props.hover.unwrap_or(false) {
        "dropdown-hover"
    } else {
        ""
    };

    let (container_classes, label_classes) = if props.link_style.unwrap_or(false) {
        (
            format!(
                "dropdown {} {} {} m-1",
                props.class.clone().unwrap_or_default(),
                direction,
                hover_class
            ),
            "link link-hover w-full flex flex-nowrap justify-between",
        )
    } else {
        (
            format!(
                "dropdown {} {} {}",
                props.class.clone().unwrap_or_default(),
                direction,
                hover_class
            ),
            "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
        )
    };

    rsx!(
        div { class: "{container_classes}",
            label {
                tabindex: "0",
                class: "{label_classes}",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", class: "mr-2", width: "16" }
                }
                span { class: "truncate", "{props.button_text}" }
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", class: "ml-2", width: "12" }
                } else if props.carat.unwrap_or(false) {
                    svg {
                        class: "ml-2 w-3 h-3 inline-block",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 20 20",
                        fill: "currentColor",
                        path { d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z", fill_rule: "evenodd", clip_rule: "evenodd" }
                    }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
                {props.children}
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownLinkProps {
    href: String,
    target: Option<String>,
    popover_target: Option<String>,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn DropDownLink(props: DropDownLinkProps) -> Element {
    let class = format!("dropdown-item {}", props.class.unwrap_or_default());

    if let Some(trigger) = &props.popover_target {
        rsx!(
            li {
                a {
                    class: "{class}",
                    "data-target": "{trigger}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    } else {
        rsx!(
            li {
                a {
                    class: "{class}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children}
                }
            }
        )
    }
}
