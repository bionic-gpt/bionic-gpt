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
        div {
            class: "dropdown {class} {direction}",
            label {
                tabindex: "0", 
                class: "btn btn-default btn-sm m-1 w-full flex justify-between",
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
                    span {
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
                tabindex: "0", 
                class: "dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 {direction}",
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
            li {
                a {
                    class: "{class}",
                    "data-drawer-target": "{trigger}",
                    target: "{target}",
                    href: "{cx.props.href}",
                    &cx.props.children,
                }
            }
        ))
    } else {
        cx.render(rsx!(
            li {
                a {
                    class: "{class}",
                    target: "{target}",
                    href: "{cx.props.href}",
                    &cx.props.children,
                }
            }
        ))
    }
}
