#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxScheme {
    #[default]
    Default,
    Primary,
    Outline,
    Danger,
}

impl CheckBoxScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            CheckBoxScheme::Default => "checkbox-default",
            CheckBoxScheme::Primary => "checkbox-primary",
            CheckBoxScheme::Outline => "checkbox-outline",
            CheckBoxScheme::Danger => "checkbox-warning",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckBoxSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl CheckBoxSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            CheckBoxSize::Default => "checkbox-sm",
            CheckBoxSize::ExtraSmall => "checkbox-xs",
            CheckBoxSize::Small => "checkbox-sm",
            CheckBoxSize::Medium => "checkbox-md",
            CheckBoxSize::Large => "checkbox-lg",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CheckBoxProps {
    children: Element,
    id: Option<String>,
    checked: Option<bool>,
    class: Option<String>,
    name: String,
    value: String,
    checkbox_size: Option<CheckBoxSize>,
    checkbox_scheme: Option<CheckBoxScheme>,
}

pub fn CheckBox(props: CheckBoxProps) -> Element {
    let checkbox_scheme = if props.checkbox_scheme.is_some() {
        props.checkbox_scheme.unwrap()
    } else {
        Default::default()
    };

    let checkbox_size = if props.checkbox_size.is_some() {
        props.checkbox_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let checked = if let Some(checked) = props.checked {
        if checked {
            Some("checked")
        } else {
            None
        }
    } else {
        None
    };

    let class = format!(
        "checkbox {} {} {}",
        class,
        checkbox_scheme.to_string(),
        checkbox_size.to_string()
    );

    rsx!(
        input {
            "type": "checkbox",
            class: "{class}",
            id: props.id,
            name: props.name,
            value: props.value,
            checked: checked,
            {props.children},
        }
    )
}
