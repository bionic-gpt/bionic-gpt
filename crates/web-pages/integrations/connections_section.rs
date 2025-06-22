#![allow(non_snake_case)]
use super::api_key_cards::ApiKeyCards;
use super::api_key_form::ApiKeyForm;
use super::oauth2_cards::Oauth2Cards;
use crate::routes;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{ApiKeyConnection, Oauth2Connection};
use dioxus::prelude::*;
use integrations::bionic_openapi::BionicOpenAPI;
use integrations::OAuth2Config;

#[component]
pub fn ConnectionsSection(
    team_id: i32,
    integration_id: i32,
    rbac: Rbac,
    openapi: BionicOpenAPI,
    api_key_connections: Vec<ApiKeyConnection>,
    oauth2_connections: Vec<Oauth2Connection>,
    oauth_client_configured: bool,
) -> Element {
    let has_api_key = openapi.has_api_key_security();
    let has_oauth2 = openapi.has_oauth2_security();

    if !has_api_key && !has_oauth2 {
        return rsx! {
            div {
                class: "mt-8",
                h3 {
                    class: "text-lg font-medium",
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

            if has_api_key {
                div {
                    class: "mb-6",
                    div {
                        class: "flex justify-between items-center mb-3",
                        h3 {
                            class: "text-lg font-medium",
                            "API Keys"
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
                            class: "bg-base-100 border border-base-300 rounded-lg p-4",
                            p {
                                class: "text-base-content/70 text-center",
                                "No API key connections configured"
                            }
                        }
                    } else {
                        {ApiKeyCards(team_id, integration_id, api_key_connections)}
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
                        if oauth_client_configured {
                            Button {
                                button_type: ButtonType::Link,
                                href: routes::integrations::Connect { team_id, integration_id }.to_string(),
                                button_style: ButtonStyle::Outline,
                                button_scheme: ButtonScheme::Primary,
                                "Add OAuth2 Connection"
                            }
                        } else {
                            Button {
                                popover_target: format!("missing-oauth-client-{}", integration_id),
                                button_style: ButtonStyle::Outline,
                                button_scheme: ButtonScheme::Primary,
                                "Add OAuth2 Connection"
                            }
                        }
                    }

                    if oauth2_connections.is_empty() {
                        div {
                            class: "bg-base-100 border border-base-300 rounded-lg p-4",
                            p {
                                class: "text-base-content/70 text-center",
                                "No OAuth2 connections configured"
                            }
                        }
                    } else {
                        {Oauth2Cards(team_id, integration_id, oauth2_connections)}
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
        if has_oauth2 && !oauth_client_configured {
            MissingOauthClientModal {
                trigger_id: format!("missing-oauth-client-{}", integration_id),
                oauth2_config: openapi.get_oauth2_config()
            }
        }
    }
}

#[component]
fn MissingOauthClientModal(trigger_id: String, oauth2_config: Option<OAuth2Config>) -> Element {
    let authorization_url = if let Some(oauth2_config) = oauth2_config {
        oauth2_config.authorization_url
    } else {
        "NOT FOUND".to_string()
    };
    rsx! {
        Modal {
            trigger_id: &trigger_id,
            ModalBody {
                h3 { class: "font-bold text-lg mb-4", "OAuth2 Client Not Configured" }
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        p { "Your sys admin needs to setup an Oauth2 Client for {authorization_url}" }
                    }
                }
            }
        }
    }
}
