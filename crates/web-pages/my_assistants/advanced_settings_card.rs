#![allow(non_snake_case)]
use crate::my_assistants::display_field::DisplayField;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AdvancedSettingsCard(prompt: db::SinglePrompt) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            CardHeader {
                title: "Advanced Settings"
            }
            CardBody {
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                    DisplayField {
                        label: "Temperature".to_string(),
                        value: format!("{}", prompt.temperature.unwrap_or(0.0)),
                    }

                    DisplayField {
                        label: "Max History Items".to_string(),
                        value: prompt.max_history_items.to_string(),
                    }

                    DisplayField {
                        label: "Max Tokens".to_string(),
                        value: prompt.max_tokens.to_string(),
                    }

                    DisplayField {
                        label: "Max Chunks".to_string(),
                        value: prompt.max_chunks.to_string(),
                    }

                    DisplayField {
                        label: "Trim Ratio".to_string(),
                        value: format!("{}%", prompt.trim_ratio),
                    }
                }
            }
        }
    }
}
