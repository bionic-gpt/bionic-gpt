use crate::shared::integrations::IntegrationWithAuthInfo;
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub enum TargetRoute {
    Assistants,
    Automations,
}

#[component]
pub fn ConnectionModal(
    team_id: String,
    prompt_id: i32,
    integration_info: IntegrationWithAuthInfo,
    target: TargetRoute,
) -> Element {
    let integration = &integration_info.integration;

    let action = match target {
        TargetRoute::Assistants => crate::routes::prompts::AddIntegration {
            team_id,
            prompt_id,
            integration_id: integration.id,
        }
        .to_string(),
        TargetRoute::Automations => crate::routes::automations::AddIntegration {
            team_id,
            prompt_id,
            integration_id: integration.id,
        }
        .to_string(),
    };

    if !integration_info.requires_api_key && !integration_info.requires_oauth2 {
        return rsx!(
            form {
                action: action.clone(),
                method: "post",
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    button_size: ButtonSize::Small,
                    "Add"
                }
            }
        );
    }

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
            submit_action: action,
            ModalBody {
                if integration_info.requires_api_key {
                    Fieldset {
                        legend: "Please select an API Key",
                        legend_class: "mt-4",
                        help_text: "This is the API key setup in the integration screen",
                        Select {
                            name: "api_connection_id",
                            {integration_info.api_key_connections.iter().map(|connection| rsx!(
                                SelectOption {
                                    value: "{connection.id}",
                                    "{connection.id}"
                                }
                            ))}
                        }
                    }
                }
                if integration_info.requires_oauth2 {
                    Fieldset {
                        legend: "Please select an OAuth2 connection",
                        legend_class: "mt-4",
                        help_text: "This is the OAuth2 key setup in the integration screen",
                        Select {
                            name: "oauth2_connection_id",
                            {integration_info.oauth2_connections.iter().map(|connection| rsx!(
                                SelectOption {
                                    value: "{connection.id}",
                                    "{connection.id}"
                                }
                            ))}
                        }
                    }
                }
                ModalAction {
                    Button {
                        class: "cancel-modal",
                        button_scheme: ButtonScheme::Warning,
                        button_size: ButtonSize::Small,
                        "Cancel"
                    }
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
