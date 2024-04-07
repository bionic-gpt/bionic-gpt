#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextAreaSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl TextAreaSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            TextAreaSize::Default => "textarea-sm",
            TextAreaSize::Small => "textarea-sm",
            TextAreaSize::ExtraSmall => "textarea-xs",
            TextAreaSize::Large => "textarea-lg",
            TextAreaSize::Medium => "textarea-md",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    children: Element,
    area_size: Option<TextAreaSize>,
    pub name: String,
    pub id: Option<String>,
    pub class: Option<String>,
    pub rows: Option<String>,
    pub label_class: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub help_text: Option<String>,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

pub fn TextArea(props: Props) -> Element {
    let input_size = if props.area_size.is_some() {
        props.area_size.unwrap()
    } else {
        Default::default()
    };

    let class = if props.class.is_some() {
        format!("{} {}", props.class.unwrap(), input_size.to_string())
    } else {
        input_size.to_string().to_string()
    };

    let value = props.value.unwrap_or("".to_string());

    let placeholder = if props.placeholder.is_some() {
        props.placeholder.unwrap()
    } else {
        "".to_string()
    };

    let label_class = if let Some(label_class) = props.label_class {
        label_class
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

    let id = if let Some(id) = props.id {
        id
    } else {
        "".to_string()
    };

    rsx!(
        match props.label {
            Some(l) => rsx!(
                label {
                    class: "{label_class}",
                    "{l}"
                }
            ),
            None => None
        }
        textarea {
            id: "{id}",
            class: "textarea textarea-bordered textarea-sm {class}",
            value: "{value}",
            name: "{props.name}",
            placeholder: "{placeholder}",
            required: props.required,
            disabled: disabled,
            readonly: props.readonly,
            rows: props.rows,
            {props.children},
        }
        match props.help_text {
            Some(l) => rsx!(
                span {
                    class: "note mb-3",
                    "{l}"
                }
            ),
            None => None
        }
    )
}
