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

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    children: Element,
    select_size: Option<SelectSize>,
    pub name: String,
    pub id: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub label_class: Option<String>,
    pub help_text: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub multiple: Option<bool>,
}

pub fn Select(props: SelectProps) -> Element {
    let select_size = if props.select_size.is_some() {
        props.select_size.unwrap()
    } else {
        Default::default()
    };

    let value = props.value.unwrap_or("".to_string());

    let class = select_size.to_string();

    let disabled = if let Some(disabled) = props.disabled {
        if disabled {
            Some(true)
        } else {
            None
        }
    } else {
        None
    };

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
        select {
            id: props.id,
            required: props.required,
            disabled: disabled,
            multiple: props.multiple,
            class: "select select-bordered {class}",
            value: "{value}",
            name: "{props.name}",
            {props.children}
        }
        match props.help_text {
            Some(l) => rsx!(
                label {
                    class: "label-text-alt",
                    span {
                        "{l}"
                    }
                }
            ),
            None => None
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct OptionProps {
    children: Element,
    pub value: String,
    pub selected_value: Option<String>,
}

pub fn SelectOption(props: OptionProps) -> Element {
    if let Some(selected) = props.selected_value {
        if selected == props.value {
            return rsx!(
                option {
                    value: props.value,
                    selected: true,
                    {props.children}
                }
            );
        }
    }
    rsx!(
        option {
            value: props.value,
            {props.children}
        }
    )
}
