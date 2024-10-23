#![allow(non_snake_case)]
#![allow(unused_braces)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarType {
    Team,
    #[default]
    User,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl AvatarSize {
    pub fn to_string(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            AvatarSize::Small => ("24", "24", "w-8 h-8"),
            AvatarSize::Medium => ("48", "48", "w-16 h-16"),
            AvatarSize::Large => ("96", "96", "w-20 w-20"),
            AvatarSize::ExtraLarge => ("128", "128", "w-32 h-32"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    avatar_size: Option<AvatarSize>,
    avatar_type: Option<AvatarType>,
    name: Option<String>,
    _email: Option<String>,
    image_src: Option<String>,
}

pub fn Avatar(props: AvatarProps) -> Element {
    let avatar_size = if props.avatar_size.is_some() {
        props.avatar_size.unwrap()
    } else {
        Default::default()
    };
    let avatar_size = avatar_size.to_string();

    let mut the_name = "?".to_string();
    if let Some(name) = props.name {
        the_name = if let Some(chr) = name.chars().next() {
            chr.to_string()
        } else {
            "?".to_string()
        };
    }

    if let Some(image) = props.image_src {
        rsx!(
            div {
                class: "avatar",
                div {
                    class: "rounded {avatar_size.2}",
                    img {
                        width: avatar_size.0,
                        height: avatar_size.1,
                        src: image
                    }
                }
            }
        )
    } else {
        match props.avatar_type {
            Some(AvatarType::User) => rsx!(
                div {
                    class: "avatar",
                    div {
                        class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            "viewbox": "0 0 27 27",
                            rect {
                                fill: "rgb(125, 73, 193)",
                                height: "27",
                                rx: "12",
                                width: "27",
                                x: "0",
                                y: "0"
                            }
                            g {
                                fill: "#fff",
                                opacity: ".5",
                                circle {
                                    cx: "13.5",
                                    cy: "30",
                                    r: "13"
                                }
                                circle {
                                    cx: "13.5",
                                    cy: "11",
                                    r: "5"
                                }
                            }
                        }
                    }
                }
            ),
            Some(_) => rsx!(
                div {
                    class: "avatar",
                    div {
                        class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 50 50",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            rect {
                                fill: "rgb(46, 77, 172)",
                                height: "100%",
                                width: "100%",
                            }
                            text {
                                fill: "#fff",
                                "font-size": "26",
                                "font-weight": "500",
                                x: "50%",
                                y: "55%",
                                "dominant-baseline": "middle",
                                "text-anchor": "middle",
                                {the_name}
                            }
                        }
                    }
                }
            ),
            None => rsx!(
                div {
                    class: "avatar",
                    div {
                        class: "rounded {avatar_size.2}",
                        svg {
                            "aria-hidden": true,
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 50 50",
                            height: avatar_size.0,
                            width: avatar_size.1,
                            rect {
                                fill: "rgb(46, 77, 172)",
                                height: "100%",
                                width: "100%",
                            }
                            text {
                                fill: "#fff",
                                "font-size": "26",
                                "font-weight": "500",
                                x: "50%",
                                y: "55%",
                                "dominant-baseline": "middle",
                                "text-anchor": "middle",
                                {the_name}
                            }
                        }
                    }
                }
            ),
        }
    }
}
