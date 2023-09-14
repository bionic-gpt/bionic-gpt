#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AlertColor {
    #[default]
    Default,
    Warn,
    Error,
    Success,
}

impl AlertColor {
    pub fn to_string(&self) -> &'static str {
        match self {
            AlertColor::Default => "flash",
            AlertColor::Warn => "flash flash-warn",
            AlertColor::Error => "flash flash-error",
            AlertColor::Success => "flash flash-success",
        }
    }
}

#[derive(Props)]
pub struct AlertProps<'a> {
    children: Element<'a>,
    class: Option<&'a str>,
    alert_color: Option<AlertColor>,
}

pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element {
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
            &cx.props.children,
        }
    ))
}
