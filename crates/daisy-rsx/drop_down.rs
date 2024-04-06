#![allow(non_snake_case)]
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

impl Direction {
    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::None => "",
            Direction::Top => "dropdown-top",
            Direction::Bottom => "dropdown-bottom",
            Direction::Left => "dropdown-left",
            Direction::Right => "dropdown-right",
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

pub fn DropDown(props: DropDownProps) -> Element {
    let direction = if let Some(direction) = props.direction {
        direction.to_string()
    } else {
        Direction::default().to_string()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    rsx!(
        div {
            class: "dropdown {class} {direction}",
            label {
                tabindex: "0",
                class: "btn btn-default btn-sm m-1 w-full flex flex-nowrap justify-between",
                "aria-haspopup": "true",
                if let Some(img_src) = props.prefix_image_src {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "16"
                        }
                },
                span {
                    class: "text-ellipsis overflow-hidden",
                    "{props.button_text}"
                }
                if let Some(img_src) = props.suffix_image_src {
                        img {
                            src: "{img_src}",
                            class: "ml-2",
                            width: "12"
                        }
                } else if props.carat.is_some() && props.carat.unwrap() {
                        div {
                            class: "dropdown-caret"
                        }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
                {props.children},
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct DropDownLinkProps {
    href: String,
    target: Option<String>,
    drawer_trigger: Option<String>,
    class: Option<String>,
    children: Element,
}

pub fn DropDownLink(props: DropDownLinkProps) -> Element {
    let class = if let Some(class) = props.class {
        format!("dropdown-item {} ", class)
    } else {
        "dropdown-item".to_string()
    };

    if let Some(trigger) = &props.drawer_trigger {
        rsx!(
            li {
                a {
                    class: "{class}",
                    "data-drawer-target": "{trigger}",
                    target: props.target,
                    href: "{props.href}",
                    {props.children},
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
                    {props.children},
                }
            }
        )
    }
}
