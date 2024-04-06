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

#[derive(Props, Clone, PartialEq)]
pub struct ToolTipProps {
    text: String,
    children: Element,
    class: Option<String>,
    alert_color: Option<ToolTipColor>,
}

pub fn ToolTip(props: ToolTipProps) -> Element {
    let alert_color = if props.alert_color.is_some() {
        props.alert_color.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let class = format!("{} {}", alert_color.to_string(), class);

    rsx!(
        div {
            class: "{class}",
            "data-tip": props.text,
            {props.children}
        }
    )
}
