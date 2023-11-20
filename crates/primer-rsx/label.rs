#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelContrast {
    Primary,
    #[default]
    Secondary,
}

impl LabelContrast {
    pub fn to_string(&self) -> &'static str {
        match self {
            LabelContrast::Primary => "Label--primary",
            LabelContrast::Secondary => "Label--secondary",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LabelColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Ghost,
}

impl LabelColor {
    pub fn to_string(&self) -> &'static str {
        match self {
            LabelColor::Default => "",
            LabelColor::Neutral => "badge-neutral",
            LabelColor::Primary => "badge-primary",
            LabelColor::Secondary => "badge-secondary",
            LabelColor::Accent => "badge-accent",
            LabelColor::Ghost => "badge-ghost",
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

#[derive(Props)]
pub struct LabelProps<'a> {
    children: Element<'a>,
    class: Option<&'a str>,
    label_contrast: Option<LabelContrast>,
    label_color: Option<LabelColor>,
    label_size: Option<LabelSize>,
}

pub fn Label<'a>(cx: Scope<'a, LabelProps<'a>>) -> Element {
    let label_contrast = if cx.props.label_contrast.is_some() {
        cx.props.label_contrast.unwrap()
    } else {
        Default::default()
    };

    let label_color = if cx.props.label_color.is_some() {
        cx.props.label_color.unwrap()
    } else {
        Default::default()
    };

    let label_size = if cx.props.label_size.is_some() {
        cx.props.label_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = cx.props.class {
        class
    } else {
        ""
    };

    let class = format!(
        "badge {} {} {} {}",
        label_contrast.to_string(),
        label_color.to_string(),
        label_size.to_string(),
        class
    );

    cx.render(rsx!(
        button {
            class: "{class}",
            &cx.props.children,
        }
    ))
}
