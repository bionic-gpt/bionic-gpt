#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DisplayFieldProps {
    pub label: String,
    pub value: String,
    #[props(optional)]
    pub custom_class: Option<String>,
    #[props(optional)]
    pub value_class: Option<String>,
    #[props(optional)]
    pub show_if: Option<bool>,
}

#[component]
pub fn DisplayField(props: DisplayFieldProps) -> Element {
    let show = props.show_if.unwrap_or(true);

    if !show {
        return rsx! { div {} };
    }

    let base_value_class = "text-sm text-gray-900 bg-gray-50 p-2 rounded border";
    let value_class = if let Some(custom) = &props.value_class {
        format!("{} {}", base_value_class, custom)
    } else {
        base_value_class.to_string()
    };

    rsx! {
        div {
            class: props.custom_class.as_deref().unwrap_or(""),
            label {
                class: "block text-sm font-medium text-gray-700 mb-1",
                "{props.label}"
            }
            p {
                class: "{value_class}",
                "{props.value}"
            }
        }
    }
}
