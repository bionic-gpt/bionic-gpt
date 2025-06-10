#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn IntegrationsCard(
    team_id: i32,
    prompt_id: i32,
    integrations: Vec<db::PromptIntegration>,
) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            div {
                class: "card-header flex justify-between items-center p-4 border-b",
                h3 {
                    class: "text-lg font-semibold",
                    "Connected Integrations"
                }
                Button {
                    button_type: ButtonType::Link,
                    href: crate::routes::prompts::ManageIntegrations{team_id, prompt_id}.to_string(),
                    button_scheme: ButtonScheme::Primary,
                    button_size: ButtonSize::Small,
                    "Manage Integrations"
                }
            }
            CardBody {
                if integrations.is_empty() {
                    p {
                        class: "text-gray-600",
                        "No integrations connected to this assistant."
                    }
                } else {
                    div {
                        class: "space-y-2",
                        for integration in integrations {
                            div {
                                class: "flex items-center justify-between p-3 bg-gray-50 rounded-lg border",
                                div {
                                    class: "flex flex-col",
                                    span {
                                        class: "font-medium text-gray-900",
                                        "{integration.name}"
                                    }
                                    span {
                                        class: "text-sm text-gray-500",
                                        "{integration.integration_type:?}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
