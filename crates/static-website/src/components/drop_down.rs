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
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
}

#[component]
pub fn DropDown(props: DropDownProps) -> Element {
    let direction = props.direction.unwrap_or_default();

    rsx!(
        div { class: "dropdown {props.class.clone().unwrap_or_default()} {direction}",
            label {
                tabindex: "0",
                class: "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                    img { src: "{img_src}", class: "mr-2", width: "16" }
                }
                span { class: "truncate", "{props.button_text}" }
                if let Some(img_src) = props.suffix_image_src {
                    img { src: "{img_src}", class: "ml-2", width: "12" }
                } else if props.carat.is_some() && props.carat.unwrap() {
                    div { class: "dropdown-caret" }
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
