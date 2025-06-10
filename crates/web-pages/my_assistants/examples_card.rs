#![allow(non_snake_case)]
use crate::my_assistants::display_field::DisplayField;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn ExamplesCard(
    disclaimer: String,
    example1: String,
    example2: String,
    example3: String,
    example4: String,
) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            CardHeader {
                title: "Examples & Disclaimer"
            }
            CardBody {
                div {
                    class: "space-y-4",

                    DisplayField {
                        label: "Disclaimer".to_string(),
                        value: disclaimer,
                    }

                    DisplayField {
                        label: "Example 1".to_string(),
                        value: example1.clone(),
                        show_if: Some(!example1.is_empty()),
                    }

                    DisplayField {
                        label: "Example 2".to_string(),
                        value: example2.clone(),
                        show_if: Some(!example2.is_empty()),
                    }

                    DisplayField {
                        label: "Example 3".to_string(),
                        value: example3.clone(),
                        show_if: Some(!example3.is_empty()),
                    }

                    DisplayField {
                        label: "Example 4".to_string(),
                        value: example4.clone(),
                        show_if: Some(!example4.is_empty()),
                    }
                }
            }
        }
    }
}
