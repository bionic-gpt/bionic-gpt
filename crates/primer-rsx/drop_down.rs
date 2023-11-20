#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    #[default]
    None,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Direction {
    pub fn to_string(&self) -> &'static str {
        match self {
            Direction::None => "dropdown-menu ",
            Direction::NorthEast => "dropdown-menu dropdown-menu-ne",
            Direction::East => "dropdown-menu dropdown-menu-e",
            Direction::SouthEast => "dropdown-menu dropdown-menu-se",
            Direction::South => "dropdown-menu dropdown-menu-s",
            Direction::SouthWest => "dropdown-menu dropdown-menu-sw",
            Direction::West => "dropdown-menu dropdown-menu-w",
        }
    }
}

#[derive(Props)]
pub struct DropDownProps<'a> {
    children: Element<'a>,
    carat: Option<bool>,
    button_text: &'a str,
    class: Option<&'a str>,
    direction: Option<Direction>,
    prefix_image_src: Option<&'a str>,
    suffix_image_src: Option<&'a str>,
}

pub fn DropDown<'a>(cx: Scope<'a, DropDownProps<'a>>) -> Element {
    let direction = if let Some(direction) = cx.props.direction {
        direction.to_string()
    } else {
        Direction::default().to_string()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    cx.render(rsx!(
        details {
            class: "dropdown details-reset details-overlay d-inline-block {class}",
            summary {
                class: "btn flex justify-between w-full items-center",
                "aria-haspopup": "true",
                if let Some(img_src) = cx.props.prefix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "16"
                        }
                    })
                } else {
                    None
                },
                span {
                    class: "Truncate",
                    span {
                        class: "Truncate-text",
                        "{cx.props.button_text}"
                    }
                }
                if let Some(img_src) = cx.props.suffix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "ml-2",
                            width: "12"
                        }
                    })
                } else if cx.props.carat.is_some() && cx.props.carat.unwrap() {
                    cx.render(rsx! {
                        div {
                            class: "dropdown-caret"
                        }
                    })
                } else {
                    None
                }
            }
            ul {
                class: "{direction}",
                &cx.props.children,
            }
        }
    ))
}

#[derive(Props)]
pub struct DropDownLinkProps<'a> {
    href: &'a str,
    target: Option<&'a str>,
    drawer_trigger: Option<String>,
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn DropDownLink<'a>(cx: Scope<'a, DropDownLinkProps<'a>>) -> Element {
    let class = if let Some(class) = cx.props.class {
        format!("dropdown-item {} ", class)
    } else {
        "dropdown-item".to_string()
    };

    let target = if let Some(target) = cx.props.target {
        target
    } else {
        ""
    };

    if let Some(trigger) = &cx.props.drawer_trigger {
        cx.render(rsx!(
            a {
                class: "{class}",
                "data-drawer-target": "{trigger}",
                target: "{target}",
                href: "{cx.props.href}",
                &cx.props.children,
            }
        ))
    } else {
        cx.render(rsx!(
            a {
                class: "{class}",
                target: "{target}",
                href: "{cx.props.href}",
                &cx.props.children,
            }
        ))
    }
}
