#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Primary,
    Outline,
    Danger,
}

impl ButtonScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonScheme::Default => "",
            ButtonScheme::Primary => "btn-primary",
            ButtonScheme::Outline => "btn-outline",
            ButtonScheme::Danger => "btn-danger",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl ButtonType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    Large,
}

impl ButtonSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonSize::Default => "",
            ButtonSize::Small => "btn-sm",
            ButtonSize::Large => "btn-large",
        }
    }
}

#[derive(Props)]
pub struct BoxProps<'a> {
    children: Element<'a>,
    id: Option<&'a str>,
    class: Option<&'a str>,
    prefix_image_src: Option<&'a str>,
    suffix_image_src: Option<&'a str>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    drawer_trigger: Option<&'a str>,
}

pub fn Button<'a>(cx: Scope<'a, BoxProps<'a>>) -> Element {
    let button_scheme = if cx.props.button_scheme.is_some() {
        cx.props.button_scheme.unwrap()
    } else {
        Default::default()
    };

    let button_type = if cx.props.button_type.is_some() {
        cx.props.button_type.unwrap()
    } else {
        Default::default()
    };
    let button_type = button_type.to_string();

    let button_size = if cx.props.button_size.is_some() {
        cx.props.button_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let id = if let Some(id) = cx.props.id { id } else { "" };

    let class = format!(
        "btn {} {} {}",
        class,
        button_scheme.to_string(),
        button_size.to_string()
    );

    if let Some(trigger) = cx.props.drawer_trigger {
        cx.render(rsx!(
            button {
                class: "{class}",
                id: "{id}",
                "data-drawer-target": "{trigger}",
                "type": "{button_type}",
                if let Some(img_src) = cx.props.prefix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                    })
                } else {
                    None
                },
                &cx.props.children,
                if let Some(img_src) = cx.props.suffix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "ml-2",
                            width: "12"
                        }
                    })
                } else {
                    None
                }
            }
        ))
    } else {
        cx.render(rsx!(
            button {
                class: "{class}",
                id: "{id}",
                "type": "{button_type}",
                if let Some(img_src) = cx.props.prefix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                    })
                } else {
                    None
                },
                &cx.props.children,
                if let Some(img_src) = cx.props.suffix_image_src {
                    cx.render(rsx! {
                        img {
                            src: "{img_src}",
                            class: "ml-2",
                            width: "12"
                        }
                    })
                } else {
                    None
                }
            }
        ))
    }
}
