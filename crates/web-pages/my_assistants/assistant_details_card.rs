#![allow(non_snake_case)]
use crate::my_assistants::display_field::DisplayField;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AssistantDetailsCard(prompt: super::upsert::PromptForm) -> Element {
    let category_name = prompt
        .categories
        .iter()
        .find(|c| c.id == prompt.category_id)
        .map(|c| c.name.as_str())
        .unwrap_or("Unknown");

    let model_name = prompt
        .models
        .iter()
        .find(|m| m.id == prompt.model_id)
        .map(|m| m.name.as_str())
        .unwrap_or("Unknown");

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

                    DisplayField {
                        label: "Category".to_string(),
                        value: category_name.to_string(),
                    }

                    DisplayField {
                        label: "Visibility".to_string(),
                        value: prompt.visibility.to_string(),
                    }

                    DisplayField {
                        label: "Model".to_string(),
                        value: model_name.to_string(),
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
