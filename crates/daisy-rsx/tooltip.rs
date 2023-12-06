#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ToolTipColor {
    #[default]
    Default,
    Warn,
    Info,
    Error,
    Success,
}

impl ToolTipColor {
    pub fn to_string(&self) -> &'static str {
        match self {
            ToolTipColor::Default => "tooltip tooltip-info",
            ToolTipColor::Info => "tooltip tooltip-info",
            ToolTipColor::Warn => "tooltip tooltip-warning",
            ToolTipColor::Error => "tooltip tooltip-error",
            ToolTipColor::Success => "tooltip tooltip-success",
        }
    }
}

#[derive(Props)]
pub struct ToolTipProps<'a> {
    text: &'a str,
    children: Element<'a>,
    class: Option<&'a str>,
    alert_color: Option<ToolTipColor>,
}

pub fn ToolTip<'a>(cx: Scope<'a, ToolTipProps<'a>>) -> Element {
    let alert_color = if cx.props.alert_color.is_some() {
        cx.props.alert_color.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!("{} {}", alert_color.to_string(), class);

    cx.render(rsx!(
        div {
            class: "{class}",
            "data-tip": cx.props.text,
            &cx.props.children,
        }
    ))
}
