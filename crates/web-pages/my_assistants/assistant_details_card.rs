#![allow(non_snake_case)]
use crate::my_assistants::display_field::DisplayField;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AssistantDetailsCard(prompt: db::SinglePrompt) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            CardHeader {
                title: "Assistant Details"
            }
            CardBody {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                    DisplayField {
                        label: "Assistant Name".to_string(),
                        value: prompt.name.clone(),
                    }

                    /***DisplayField {
                        label: "Category".to_string(),
                        value: category_name.to_string(),
                    }

                    DisplayField {
                        label: "Visibility".to_string(),
                        value: prompt.visibility.to_string(),
                    }**/

                    DisplayField {
                        label: "Model".to_string(),
                        value: prompt.model_name.to_string(),
                    }
                }

                div {
                    class: "mt-6",
                    DisplayField {
                        label: "Description".to_string(),
                        value: prompt.description.clone(),
                        value_class: Some("p-3".to_string()),
                    }
                }
            }
        }
    }
}
