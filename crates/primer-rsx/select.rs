#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SelectSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl SelectSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            SelectSize::Default => "select-sm",
            SelectSize::Small => "select-sm",
            SelectSize::ExtraSmall => "select-xs",
            SelectSize::Large => "select-lg",
            SelectSize::Medium => "select-md",
        }
    }
}

#[derive(Props)]
pub struct SelectProps<'a> {
    children: Element<'a>,
    select_size: Option<SelectSize>,
    pub name: &'a str,
    pub id: Option<&'a str>,
    pub value: Option<&'a str>,
    pub label: Option<&'a str>,
    pub label_class: Option<&'a str>,
    pub help_text: Option<&'a str>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub multiple: Option<bool>,
}

pub fn Select<'a>(cx: Scope<'a, SelectProps<'a>>) -> Element {
    let select_size = if cx.props.select_size.is_some() {
        cx.props.select_size.unwrap()
    } else {
        Default::default()
    };

    let value = cx.props.value.unwrap_or("");

    let class = select_size.to_string();

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
        select {
            id: cx.props.id,
            required: cx.props.required,
            disabled: cx.props.disabled,
            multiple: cx.props.multiple,
            class: "select select-bordered {class}",
            value: "{value}",
            name: "{cx.props.name}",
            &cx.props.children
        }
        match cx.props.help_text {
            Some(l) => cx.render(rsx!(
                label {
                    class: "label-text-alt",
                    span {
                        "{l}"
                    }
                }
            )),
            None => None
        }
    ))
}

#[derive(Props)]
pub struct OptionProps<'a> {
    children: Element<'a>,
    pub value: &'a str,
    pub selected_value: Option<&'a str>,
}

pub fn SelectOption<'a>(cx: Scope<'a, OptionProps<'a>>) -> Element {
    if let Some(selected) = cx.props.selected_value {
        if selected == cx.props.value {
            return cx.render(rsx!(
                option {
                    value: cx.props.value,
                    selected: true,
                    &cx.props.children
                }
            ));
        }
    }
    cx.render(rsx!(
        option {
            value: cx.props.value,
            &cx.props.children
        }
    ))
}
