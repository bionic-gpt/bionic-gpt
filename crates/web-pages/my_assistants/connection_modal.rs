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
        input {
            "type": "checkbox",
            id: "{modal_id}",
            class: "modal-toggle"
        }
        div {
            class: "modal",
            div {
                class: "modal-box",
                h3 {
                    class: "font-bold text-lg",
                    "Add {integration.name} Integration"
                }

                form {
                    action: crate::routes::prompts::AddIntegration {
                        team_id,
                        prompt_id,
                        integration_id: integration.id
                    }.to_string(),
                    method: "post",

                    div {
                        class: "py-4 space-y-4",

                        // API Key connection dropdown
                        if integration_info.requires_api_key {
                            if let Some(connections) = available_connections.get(&integration.id) {
                                if !connections.api_key_connections.is_empty() {
                                    div {
                                        class: "form-control w-full",
                                        label {
                                            class: "label",
                                            span {
                                                class: "label-text",
                                                "API Key Connection"
                                            }
                                        }
                                        select {
                                            class: "select select-bordered w-full",
                                            name: "api_connection_id",
                                            required: true,
                                            option {
                                                value: "",
                                                "Select an API Key Connection"
                                            }
                                            for connection in &connections.api_key_connections {
                                                option {
                                                    value: "{connection.id}",
                                                    "Connection {connection.id} (Created: {connection.created_at})"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // OAuth2 connection dropdown
                        if integration_info.requires_oauth2 {
                            if let Some(connections) = available_connections.get(&integration.id) {
                                if !connections.oauth2_connections.is_empty() {
                                    div {
                                        class: "form-control w-full",
                                        label {
                                            class: "label",
                                            span {
                                                class: "label-text",
                                                "OAuth2 Connection"
                                            }
                                        }
                                        select {
                                            class: "select select-bordered w-full",
                                            name: "oauth2_connection_id",
                                            required: true,
                                            option {
                                                value: "",
                                                "Select an OAuth2 Connection"
                                            }
                                            for connection in &connections.oauth2_connections {
                                                option {
                                                    value: "{connection.id}",
                                                    "Connection {connection.id} (Created: {connection.created_at})"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "modal-action",
                        label {
                            "for": "{modal_id}",
                            class: "btn btn-ghost",
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Add Integration"
                        }
                    }
                }
            }
        }
    }
}
