#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelRole {
    #[default]
    Neutral,
    Danger,
    Warning,
    Success,
    Info,
    Highlight,
}

impl LabelRole {
    pub fn to_string(&self) -> &'static str {
        match self {
            LabelRole::Neutral => "label-neutral",
            LabelRole::Danger => "label-danger",
            LabelRole::Warning => "label-warning",
            LabelRole::Success => "label-success",
            LabelRole::Info => "label-info",
            LabelRole::Highlight => "label-highlight",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelSize {
    #[default]
    Small,
    Large,
}

impl LabelSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            LabelSize::Small => "",
            LabelSize::Large => "badge-lg",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    children: Element,
    class: Option<String>,
    label_role: Option<LabelRole>,
    label_size: Option<LabelSize>,
}

pub fn Label(props: LabelProps) -> Element {
    let label_role = if props.label_role.is_some() {
        props.label_role.unwrap()
    } else {
        Default::default()
    };

    let label_size = if props.label_size.is_some() {
        props.label_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!(
        "badge {} {} {}",
        label_role.to_string(),
        label_size.to_string(),
        class
    );

    rsx!(
        button {
            class: "{class}",
            {props.children},
        }
    )
}
