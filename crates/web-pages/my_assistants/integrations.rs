#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::my_assistants::connection_modal::ConnectionModal;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{ApiKeyConnection, Integration, Oauth2Connection};
use dioxus::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationForm {
    pub prompt_id: i32,
    pub prompt_name: String,
    pub selected_integration_ids: Vec<i32>,
    pub integration_connections: HashMap<i32, ConnectionSelection>,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub integrations: Vec<IntegrationWithAuthInfo>,
    #[serde(skip)]
    pub available_connections: HashMap<i32, AvailableConnections>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConnectionSelection {
    pub api_connection_id: Option<i32>,
    pub oauth2_connection_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationWithAuthInfo {
    pub integration: Integration,
    pub requires_api_key: bool,
    pub requires_oauth2: bool,
    pub has_connections: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvailableConnections {
    pub api_key_connections: Vec<ApiKeyConnection>,
    pub oauth2_connections: Vec<Oauth2Connection>,
}

pub fn page(team_id: i32, rbac: Rbac, form: IntegrationForm) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Manage Integrations",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Assistants".into(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: "My Assistants".into(),
                            href: Some(crate::routes::prompts::MyAssistants{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: form.prompt_name.clone(),
                            href: Some(crate::routes::prompts::View{team_id, prompt_id: form.prompt_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: "Manage Integrations".into(),
                            href: None
                        }
                    ]
                }
                h3 {
                    "Manage Integrations for {form.prompt_name}"
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",

                // Display error if present
                if let Some(error) = &form.error {
                    div {
                        class: "alert alert-error mb-4",
                        "{error}"
                    }
                }

                // Integrations Section
                Card {
                    class: "mb-6",
                    CardHeader {
                        title: "Available Integrations"
                    }
                    CardBody {
                        Alert {
                            class: "mb-4",
                            "Manage which integrations this assistant can use"
                        }

                        if !form.integrations.is_empty() {
                            div {
                                class: "overflow-x-auto",
                                table {
                                    class: "table table-sm w-full",
                                    thead {
                                        tr {
                                            th { "Integration" }
                                            th { "Type" }
                                            th { "Status" }
                                            th { "Authentication" }
                                            th { "Action" }
                                        }
                                    }
                                    tbody {
                                        for integration_info in &form.integrations {
                                            tr {
                                                td { "{integration_info.integration.name}" }
                                                td { "{integration_info.integration.integration_type:?}" }
                                                td {
                                                    Label {
                                                        label_role: match integration_info.integration.integration_status {
                                                            db::IntegrationStatus::Configured => LabelRole::Success,
                                                            _ => LabelRole::Warning
                                                        },
                                                        "{integration_info.integration.integration_status:?}"
                                                    }
                                                }
                                                td {
                                                    // Auth requirements indicator
                                                    if integration_info.requires_api_key || integration_info.requires_oauth2 {
                                                        div {
                                                            class: "flex items-center gap-1",
                                                            if integration_info.requires_api_key {
                                                                Label {
                                                                    label_size: LabelSize::Small,
                                                                    "API Key"
                                                                }
                                                            }
                                                            if integration_info.requires_oauth2 {
                                                                Label {
                                                                    label_size: LabelSize::Small,
                                                                    "OAuth2"
                                                                }
                                                            }
                                                            if !integration_info.has_connections {
                                                                Label {
                                                                    label_size: LabelSize::Small,
                                                                    "No connections available ⚠️"
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Label {
                                                            label_size: LabelSize::Small,
                                                            "None"
                                                        }
                                                    }
                                                }
                                                td {
                                                    if form.selected_integration_ids.contains(&integration_info.integration.id) {
                                                        // Show Remove button
                                                        form {
                                                            action: crate::routes::prompts::RemoveIntegration {
                                                                team_id,
                                                                prompt_id: form.prompt_id,
                                                                integration_id: integration_info.integration.id
                                                            }.to_string(),
                                                            method: "post",
                                                            Button {
                                                                button_type: ButtonType::Submit,
                                                                button_scheme: ButtonScheme::Error,
                                                                button_size: ButtonSize::Small,
                                                                "Remove"
                                                            }
                                                        }
                                                    } else {
                                                        // Show Add button (with modal if connection required)
                                                        ConnectionModal {
                                                            team_id: team_id,
                                                            prompt_id: form.prompt_id,
                                                            integration_info: integration_info.clone(),
                                                            available_connections: form.available_connections.clone(),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            div {
                                class: "text-gray-500 italic text-center py-4",
                                "No integrations available"
                            }
                        }
                    }
                }

                // Navigation Actions
                Card {
                    CardBody {
                        div {
                            class: "flex justify-between",
                            Button {
                                button_type: ButtonType::Link,
                                href: crate::routes::prompts::View { team_id, prompt_id: form.prompt_id }.to_string(),
                                button_scheme: ButtonScheme::Error,
                                "Back to Assistant"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
