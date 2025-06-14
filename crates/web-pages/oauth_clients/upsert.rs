#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct OauthClientForm {
    #[validate(length(min = 1, message = "Client ID is required"))]
    pub client_id: String,
    #[validate(length(min = 1, message = "Client Secret is required"))]
    pub client_secret: String,
    #[validate(length(min = 1, message = "Provider is required"))]
    pub provider: String,
    #[validate(length(min = 1, message = "Provider URL is required"))]
    pub provider_url: String,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, oauth_client: OauthClientForm) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::OauthClients,
            team_id: team_id,
            rbac: rbac,
            title: "OAuth Clients",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "OAuth Clients".into(),
                            href: Some(crate::routes::oauth_clients::Index { team_id }.to_string())
                        },
                        BreadcrumbItem {
                            text: "New OAuth Client".into(),
                            href: None
                        }
                    ]
                }
            ),

            Card {
                CardHeader {
                    title: "Create OAuth Client"
                }
                CardBody {
                    form {
                        method: "post",
                        class: "flex flex-col space-y-4",

                        // Display error if present
                        if let Some(error) = &oauth_client.error {
                            Alert {
                                alert_color: AlertColor::Error,
                                class: "mb-4",
                                "{error}"
                            }
                        }

                        Input {
                            input_type: InputType::Text,
                            name: "provider",
                            label: "Provider",
                            help_text: "The OAuth provider name (e.g., google, github, microsoft)",
                            placeholder: "e.g., google, github, microsoft",
                            value: "{oauth_client.provider}",
                            required: true
                        }

                        Input {
                            input_type: InputType::Text,
                            name: "provider_url",
                            label: "Provider URL",
                            help_text: "The OAuth provider authorization URL",
                            placeholder: "https://accounts.google.com/o/oauth2/v2/auth",
                            value: "{oauth_client.provider_url}",
                            required: true
                        }

                        Input {
                            input_type: InputType::Text,
                            name: "client_id",
                            label: "Client ID",
                            help_text: "The client ID provided by your OAuth provider",
                            placeholder: "Enter the OAuth client ID",
                            value: "{oauth_client.client_id}",
                            required: true
                        }

                        Input {
                            input_type: InputType::Password,
                            name: "client_secret",
                            label: "Client Secret",
                            help_text: "The client secret provided by your OAuth provider",
                            placeholder: "Enter the OAuth client secret",
                            value: "{oauth_client.client_secret}",
                            required: true
                        }

                        div {
                            class: "mt-6 flex justify-between",
                            Button {
                                button_type: ButtonType::Link,
                                href: crate::routes::oauth_clients::Index { team_id }.to_string(),
                                button_scheme: ButtonScheme::Error,
                                "Cancel"
                            }
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Create OAuth Client"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
