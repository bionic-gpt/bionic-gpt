#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Number,
    Email,
    Password,
}

impl InputType {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Number => "number",
            InputType::Email => "email",
            InputType::Password => "password",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Default,
    Small,
    Large,
}

impl InputSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputSize::Default => "",
            InputSize::Small => "sm",
            InputSize::Large => "large",
        }
    }
}

#[derive(Props)]
pub struct InputProps<'a> {
    input_type: Option<InputType>,
    input_size: Option<InputSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub label_class: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub step: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    let input_type = if cx.props.input_type.is_some() {
        cx.props.input_type.unwrap()
    } else {
        Default::default()
    };

    let input_size = if cx.props.input_size.is_some() {
        cx.props.input_size.unwrap()
    } else {
        Default::default()
    };

    let input_type = input_type.to_string();
    let input_size = input_size.to_string();

    let input_class = format!("{} {}", input_type, input_size);

    cx.render(rsx!(
        match (cx.props.label, cx.props.required) {
            (Some(l), Some(_)) => cx.render(rsx!(
                label {
                    class: cx.props.label_class,
                    strong {
                        "{l} *"
                    }
                }
            )),
            (Some(l), None) => cx.render(rsx!(
                label {
                    class: cx.props.label_class,
                    strong {
                        "{l}"
                    }
                }
            )),
            (None, _) => None
        }
        input {
            id: cx.props.id,
            class: "{input_class}",
            value: cx.props.value,
            required: cx.props.required,
            disabled: cx.props.disabled,
            readonly: cx.props.readonly,
            name: "{cx.props.name}",
            placeholder: cx.props.placeholder,
            step: cx.props.step,
            "type": "{input_type}"
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                span {
                    class: "note mb-3",
                    "{l}"
                }
            )),
            None => None
        }
    ))
}
