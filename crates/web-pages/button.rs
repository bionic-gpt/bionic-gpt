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
            ButtonScheme::Default => "btn-default",
            ButtonScheme::Primary => "btn-primary",
            ButtonScheme::Outline => "btn-outline",
            ButtonScheme::Danger => "btn-warning",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    Link,
    #[default]
    Button,
}

impl ButtonType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
            ButtonType::Link => "button",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl ButtonSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonSize::Default => "btn-sm",
            ButtonSize::ExtraSmall => "btn-xs",
            ButtonSize::Small => "btn-sm",
            ButtonSize::Medium => "btn-md",
            ButtonSize::Large => "btn-lg",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonShape {
    #[default]
    None,
    Circle,
    Square,
}

impl ButtonShape {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonShape::None => "",
            ButtonShape::Circle => "btn-circle",
            ButtonShape::Square => "btn-square",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    href: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
    disabled_text: Option<String>,
    button_shape: Option<ButtonShape>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = if props.button_scheme.is_some() {
        props.button_scheme.unwrap()
    } else {
        Default::default()
    };

    let button_type = if props.button_type.is_some() {
        props.button_type.unwrap()
    } else {
        Default::default()
    };
    let button_type = button_type.to_string();

    let button_size = if props.button_size.is_some() {
        props.button_size.unwrap()
    } else {
        Default::default()
    };

    let button_shape = if props.button_shape.is_some() {
        props.button_shape.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let disabled = if let Some(disabled) = props.disabled {
        if disabled {
            Some(true)
        } else {
            None
        }
    } else {
        None
    };

    let class = format!(
        "btn {} {} {} {}",
        class,
        button_scheme.to_string(),
        button_size.to_string(),
        button_shape.to_string()
    );

    if props.button_type == Some(ButtonType::Link) {
        rsx!(
            a {
                class: "{class}",
                id: props.id,
                href: props.href,
                if let Some(img_src) = props.prefix_image_src {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                },
                {props.children},
                if let Some(img_src) = props.suffix_image_src {
                        img {
                            src: "{img_src}",
                            class: "mr-2",
                            width: "12"
                        }
                }
            }
        )
    } else {
        rsx!(
            button {
                class: "{class}",
                id: props.id,
                disabled: disabled,
                "data-drawer-target": props.drawer_trigger,
                "data-modal-target": props.modal_trigger,
                "type": "{button_type}",
                "data-disabled-text": props.disabled_text,
                if let Some(img_src) = props.prefix_image_src {
                        img {
                            src: "{img_src}",
                            class: "h-5 w-5",
                        }
                },
                {props.children},
                if let Some(img_src) = props.suffix_image_src {
                        img {
                            src: "{img_src}",
                            class: "h-5 w-5",
                        }
                }
            }
        )
    }
}
