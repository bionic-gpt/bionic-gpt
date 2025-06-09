#![allow(non_snake_case)]
use daisy_rsx::*;
use db::authz::Rbac;
use db::{ApiKeyConnection, Oauth2Connection};
use dioxus::prelude::*;
use integrations::bionic_openapi::BionicOpenAPI;

use super::api_key_connections_table::ApiKeyConnectionsTable;
use super::api_key_form::ApiKeyForm;
use super::oauth2_connections_table::Oauth2ConnectionsTable;

#[component]
pub fn ConnectionsSection(
    team_id: i32,
    integration_id: i32,
    rbac: Rbac,
    openapi: BionicOpenAPI,
    api_key_connections: Vec<ApiKeyConnection>,
    oauth2_connections: Vec<Oauth2Connection>,
) -> Element {
    let has_api_key = openapi.has_api_key_security();
    let has_oauth2 = openapi.has_oauth2_security();

    if !has_api_key && !has_oauth2 {
        return rsx! {
            div {
                class: "mt-8",
                h2 {
                    class: "font-semibold mb-4",
                    "Connections"
                }
                p {
                    class: "text-gray-500 italic",
                    "This integration does not require any connections"
                }
            }
        };
    }

    rsx! {
        div {
            class: "mt-8",
            h2 {
                class: "font-semibold mb-4",
                "Connections"
            }

            if has_api_key {
                div {
                    class: "mb-6",
                    div {
                        class: "flex justify-between items-center mb-3",
                        h3 {
                            class: "text-lg font-medium",
                            "API Key Connections"
                        }
                        Button {
                            popover_target: format!("configure-api-key-{}", integration_id),
                            button_style: ButtonStyle::Outline,
                            button_scheme: ButtonScheme::Primary,
                            "Add API Key"
                        }
                    }

                    if api_key_connections.is_empty() {
                        div {
                            class: "bg-gray-50 border border-gray-200 rounded-lg p-4",
                            p {
                                class: "text-gray-500 text-center",
                                "No API key connections configured"
                            }
                        }
                    } else {
                        {ApiKeyConnectionsTable(team_id, integration_id, api_key_connections)}
                    }
                }
            }

            if has_oauth2 {
                div {
                    class: "mb-6",
                    div {
                        class: "flex justify-between items-center mb-3",
                        h3 {
                            class: "text-lg font-medium",
                            "OAuth2 Connections"
                        }
                        Button {
                            button_style: ButtonStyle::Outline,
                            button_scheme: ButtonScheme::Primary,
                            "Add OAuth2 Connection"
                        }
                    }

                    if oauth2_connections.is_empty() {
                        div {
                            class: "bg-gray-50 border border-gray-200 rounded-lg p-4",
                            p {
                                class: "text-gray-500 text-center",
                                "No OAuth2 connections configured"
                            }
                        }
                    } else {
                        {Oauth2ConnectionsTable(team_id, integration_id, oauth2_connections)}
                    }
                }
            }
        }

        // Add API key configuration modal
        if has_api_key {
            ApiKeyForm {
                team_id,
                integration_id,
                integration_name: openapi.get_title().to_string()
            }
        }
    }
}
