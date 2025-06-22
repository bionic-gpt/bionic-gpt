#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use integrations::{BionicOpenAPI, OAuth2Config};

#[derive(Clone, PartialEq)]
pub struct IntegrationSummary {
    pub openapi: BionicOpenAPI,
    pub id: i32,
    pub api_key_count: usize,
    pub oauth2_count: usize,
    pub oauth_client_configured: bool,
}

#[component]
pub fn IntegrationCard(integration: IntegrationSummary, team_id: i32) -> Element {
    let has_oauth2 = integration.openapi.get_oauth2_config().is_some();
    let has_api_key = integration.openapi.has_api_key_security();
    let count = if has_oauth2 {
        integration.oauth2_count
    } else if has_api_key {
        integration.api_key_count
    } else {
        0
    };

    rsx! {
        Card {
            class: "p-3 mt-5 flex flex-row",
            a {
                href: crate::routes::integrations::View { team_id, id: integration.id }.to_string(),
                class: "no-underline flex-1 min-w-0",
                div {
                    class: "flex flex-row",
                    img {
                        class: "border border-neutral-content rounded p-2",
                        src: integration.openapi.get_logo_url(),
                        width: "48",
                        height: "48"
                    }
                    div {
                        class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                        h2 { class: "font-semibold", "{integration.openapi.get_title()}" }
                        p {
                            class: "truncate overflow-hidden whitespace-nowrap",
                            if let Some(description) = integration.openapi.get_description() {
                                "{description}"
                            }
                        }
                    }
                }
            }
            if has_oauth2 || has_api_key {
                div {
                    class: "flex flex-col justify-center text-center ml-4",
                    div { "{count}" }
                    div {
                        class: "text-base-content/70 text-sm",
                        if has_oauth2 {
                            "Connection"
                            if count != 1 { "s" }
                        } else if has_api_key {
                            "Key"
                            if count != 1 { "s" }
                        }
                    }
                }
            }
            if has_oauth2 {
                div {
                    class: "flex flex-col justify-center ml-4",
                    if integration.oauth_client_configured {
                        a {
                            class: "btn btn-secondary btn-sm ",
                            "data-turbo": "false",
                            href: crate::routes::integrations::Connect { team_id, integration_id: integration.id }.to_string(),
                            "Connect"
                        }
                    } else {
                        Button {
                            button_scheme: ButtonScheme::Secondary,
                            popover_target: format!("missing-oauth-client-{}", integration.id),
                            "Connect"
                        }
                    }
                }
            }
            if has_api_key {
                div {
                    class: "flex flex-col justify-center ml-4",
                    Button {
                        popover_target: format!("configure-api-key-{}", integration.id),
                        button_scheme: ButtonScheme::Secondary,
                        "Configure"
                    }
                }
            }
        }

        if integration.openapi.has_api_key_security() {
            super::api_key_form::ApiKeyForm {
                team_id,
                integration_id: integration.id,
                integration_name: integration.openapi.get_title().to_string(),
            }
        }
        if integration.openapi.get_oauth2_config().is_some() && !integration.oauth_client_configured {
            MissingOauthClientModal {
                trigger_id: format!("missing-oauth-client-{}", integration.id),
                oauth2_config: integration.openapi.get_oauth2_config()
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
