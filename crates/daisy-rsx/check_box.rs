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

#[derive(Props)]
pub struct CheckBoxProps<'a> {
    children: Element<'a>,
    id: Option<&'a str>,
    checked: Option<bool>,
    class: Option<&'a str>,
    checkbox_size: Option<CheckBoxSize>,
    checkbox_scheme: Option<CheckBoxScheme>,
}

pub fn CheckBox<'a>(cx: Scope<'a, CheckBoxProps<'a>>) -> Element {
    let checkbox_scheme = if cx.props.checkbox_scheme.is_some() {
        cx.props.checkbox_scheme.unwrap()
    } else {
        Default::default()
    };

    let checkbox_size = if cx.props.checkbox_size.is_some() {
        cx.props.checkbox_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let checked = if let Some(checked) = cx.props.checked {
        if checked {
            Some(true)
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

    cx.render(rsx!(
        input {
            "type": "checkbox",
            class: "{class}",
            id: cx.props.id,
            checked: checked,
            &cx.props.children,
        }
    ))
}
