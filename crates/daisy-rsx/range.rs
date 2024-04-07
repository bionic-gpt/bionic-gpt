#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RangeColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl RangeColor {
    pub fn to_string(&self) -> &'static str {
        match self {
            RangeColor::Default => "range range-info",
            RangeColor::Info => "range range-info",
            RangeColor::Warn => "range range-warning",
            RangeColor::Error => "range range-error",
            RangeColor::Success => "range range-success",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RangeSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl RangeSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            RangeSize::Default => "range-sm",
            RangeSize::ExtraSmall => "range-xs",
            RangeSize::Small => "range-sm",
            RangeSize::Large => "range-lg",
            RangeSize::Medium => "range-md",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RangeProps {
    children: Element,
    class: Option<String>,
    min: i32,
    max: i32,
    value: i32,
    name: String,
    label: Option<String>,
    label_class: Option<String>,
    help_text: Option<String>,
    range_color: Option<RangeColor>,
}

pub fn Range(props: RangeProps) -> Element {
    let range_color = if props.range_color.is_some() {
        props.range_color.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("{} {}", range_color.to_string(), class);

    rsx!(
        match props.label {
            Some(l) => rsx!(
                label {
                    class: props.label_class,
                    "{l}"
                }
            ),
            None => None
        }
        input {
            "type": "range",
            min: "{props.min}",
            max: "{props.max}",
            value: "{props.value}",
            class: "{class}",
            name: props.name,
            {props.children},
        }
        match props.help_text {
            Some(l) => rsx!(
                label {
                    span {
                        class: "label-text-alt",
                        "{l}"
                    }
                }
            ),
            None => None
        }
    )
}
