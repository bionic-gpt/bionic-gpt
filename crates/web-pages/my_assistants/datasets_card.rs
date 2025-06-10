#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DatasetsCard(team_id: i32, prompt_id: i32, datasets: Vec<db::PromptDataset>) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            div {
                class: "card-header flex justify-between items-center p-4 border-b",
                h3 {
                    class: "text-lg font-semibold",
                    "Connected Datasets"
                }
                Button {
                    button_type: ButtonType::Link,
                    href: crate::routes::prompts::ManageDatasets{team_id, prompt_id}.to_string(),
                    button_scheme: ButtonScheme::Primary,
                    button_size: ButtonSize::Small,
                    "Manage Datasets"
                }
            }
            CardBody {
                if datasets.is_empty() {
                    p {
                        class: "text-gray-600",
                        "No datasets connected to this assistant."
                    }
                } else {
                    div {
                        class: "space-y-2",
                        for dataset in datasets {
                            div {
                                class: "flex items-center justify-between p-3 bg-gray-50 rounded-lg border",
                                span {
                                    class: "font-medium text-gray-900",
                                    "{dataset.name}"
                                }
                                span {
                                    class: "text-sm text-gray-500",
                                    "Dataset"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
