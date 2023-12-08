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

#[derive(Props)]
pub struct RangeProps<'a> {
    children: Element<'a>,
    class: Option<&'a str>,
    min: i32,
    max: i32,
    value: i32,
    name: &'a str,
    label: Option<&'a str>,
    label_class: Option<&'a str>,
    help_text: Option<&'a str>,
    range_color: Option<RangeColor>,
}

pub fn Range<'a>(cx: Scope<'a, RangeProps<'a>>) -> Element {
    let range_color = if cx.props.range_color.is_some() {
        cx.props.range_color.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("{} {}", range_color.to_string(), class);

    cx.render(rsx!(
        match cx.props.label {
            Some(l) => cx.render(rsx!(
                label {
                    class: cx.props.label_class,
                    "{l}"
                }
            )),
            None => None
        }
        input {
            "type": "range",
            min: "{cx.props.min}",
            max: "{cx.props.max}",
            value: "{cx.props.value}",
            class: "{class}",
            name: cx.props.name,
            &cx.props.children,
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                label {
                    span {
                        class: "label-text-alt",
                        "{l}"
                    }
                }
            )),
            None => None
        }
    ))
}
