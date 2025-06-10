#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn IntegrationsCard(team_id: i32, prompt_id: i32) -> Element {
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
                p {
                    class: "text-gray-600",
                    "Click 'Manage Integrations' to view and configure integration connections for this assistant."
                }
            }
        }
    }
}
