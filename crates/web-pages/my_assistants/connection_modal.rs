use crate::my_assistants::integrations::{AvailableConnections, IntegrationWithAuthInfo};
use daisy_rsx::*;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ConnectionModal(
    team_id: i32,
    prompt_id: i32,
    integration_info: IntegrationWithAuthInfo,
    available_connections: HashMap<i32, AvailableConnections>,
) -> Element {
    let integration = &integration_info.integration;
    let modal_id = format!("add-modal-{}", integration.id);

    rsx! {
        Button {
            popover_target: modal_id.clone(),
            button_scheme: ButtonScheme::Primary,
            button_size: ButtonSize::Small,
            disabled: !integration_info.has_connections && (integration_info.requires_api_key || integration_info.requires_oauth2),
            "Add"
        }

        Modal {
            trigger_id: modal_id,
            submit_action: crate::routes::prompts::AddIntegration {
                team_id,
                prompt_id,
                integration_id: integration_info.integration.id
            }.to_string(),
            ModalBody {
                "Connect it",
                ModalAction {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        button_size: ButtonSize::Small,
                        "Connect"
                    }
                }
            }
        }
    }
}
