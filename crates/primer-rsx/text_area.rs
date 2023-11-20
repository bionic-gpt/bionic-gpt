#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextAreaSize {
    #[default]
    Default,
    Small,
    Large,
    ExtraLarge,
}

impl TextAreaSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            TextAreaSize::Default => "textarea-sm",
            TextAreaSize::Small => "textarea-xs",
            TextAreaSize::Large => "",
            TextAreaSize::ExtraLarge => "textarea-large",
        }
    }
}

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    area_size: Option<TextAreaSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub class: Option<&'a str>,
    pub rows: Option<&'a str>,
    pub label_class: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

pub fn TextArea<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let input_size = if cx.props.area_size.is_some() {
        cx.props.area_size.unwrap()
    } else {
        Default::default()
    };

    let class = if cx.props.class.is_some() {
        format!("{} {}", cx.props.class.unwrap(), input_size.to_string())
    } else {
        input_size.to_string().to_string()
    };

    let value = cx.props.value.unwrap_or("");

    let placeholder = if cx.props.placeholder.is_some() {
        cx.props.placeholder.unwrap()
    } else {
        ""
    };

    let label_class = if let Some(label_class) = cx.props.label_class {
        label_class
    } else {
        ""
    };

    let id = if let Some(id) = cx.props.id { id } else { "" };

    cx.render(rsx!(
        match cx.props.label {
            Some(l) => cx.render(rsx!(
                label {
                    class: "{label_class}",
                    "{l}"
                }
            )),
            None => None
        }
        textarea {
            id: "{id}",
            class: "textarea textarea-bordered textarea-sm {class}",
            value: "{value}",
            name: "{cx.props.name}",
            placeholder: "{placeholder}",
            required: cx.props.required,
            disabled: cx.props.disabled,
            readonly: cx.props.readonly,
            rows: cx.props.rows,
            &cx.props.children,
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                span {
                    class: "note mb-3",
                    "{l}"
                }
            )),
            None => None
        }
    ))
}
