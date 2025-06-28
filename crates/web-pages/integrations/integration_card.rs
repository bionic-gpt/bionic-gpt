#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
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

    let description = integration.openapi.get_description().unwrap_or_default();

    rsx! {
        CardItem {
            image_src: Some(integration.openapi.get_logo_url()),
            avatar_name: None,
            title: integration.openapi.get_title().to_string(),
            description: if description.is_empty() { None } else { Some(rsx!(span { "{description}" })) },
            footer: None,
            count_labels: if has_oauth2 || has_api_key {
                vec![CountLabel { count, label: if has_oauth2 { "Connection".into() } else { "Key".into() } }]
            } else {
                vec![]
            },
            action: Some(rsx!(
                if has_oauth2 {
                    if integration.oauth_client_configured {
                        super::oauth_connect_button::OauthConnectButton {
                            team_id,
                            integration_id: integration.id,
                            class: "btn btn-secondary btn-sm ".to_string(),
                            label: "Connect".to_string(),
                        }
                    } else {
                        Button {
                            button_scheme: ButtonScheme::Secondary,
                            popover_target: format!("missing-oauth-client-{}", integration.id),
                            "Connect"
                        }
                    }
                } else if has_api_key {
                    Button {
                        popover_target: format!("configure-api-key-{}", integration.id),
                        button_scheme: ButtonScheme::Secondary,
                        "Configure"
                    }
                }
            )),
            class: None,
            popover_target: None,
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
