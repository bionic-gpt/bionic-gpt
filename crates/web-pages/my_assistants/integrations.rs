#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::my_assistants::connection_modal::ConnectionModal;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{ApiKeyConnection, Integration, Oauth2Connection};
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationForm {
    pub prompt_id: i32,
    pub prompt_name: String,
    pub selected_integration_ids: Vec<i32>,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub integrations: Vec<IntegrationWithAuthInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationWithAuthInfo {
    pub integration: Integration,
    pub requires_api_key: bool,
    pub requires_oauth2: bool,
    pub has_connections: bool,
    pub api_key_connections: Vec<ApiKeyConnection>,
    pub oauth2_connections: Vec<Oauth2Connection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationStatus {
    Available,
    RequiresAPIKey,
    RequiresOauth2Key,
    Active,
}

#[component]
pub fn Status(status: IntegrationStatus) -> Element {
    match status {
        IntegrationStatus::Active => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "Active"
            }
        ),
        IntegrationStatus::RequiresAPIKey => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Warning,
                "Missing API Key"
            }
        ),
        IntegrationStatus::RequiresOauth2Key => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Warning,
                "Missing Oauth2"
            }
        ),
        IntegrationStatus::Available => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "Available"
            }
        ),
    }
}

pub fn determine_status(info: &IntegrationWithAuthInfo, connected: bool) -> IntegrationStatus {
    if connected {
        IntegrationStatus::Active
    } else if info.requires_api_key && !info.has_connections {
        IntegrationStatus::RequiresAPIKey
    } else if info.requires_oauth2 && !info.has_connections {
        IntegrationStatus::RequiresOauth2Key
    } else {
        IntegrationStatus::Available
    }
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
                                            th { "Action" }
                                        }
                                    }
                                    tbody {
                                        for integration_info in &form.integrations {
                                            tr {
                                                td { "{integration_info.integration.name}" }
                                                td { "{integration_info.integration.integration_type:?}" }
                                                td {
                                                    Status {
                                                        status: determine_status(integration_info, form.selected_integration_ids.contains(&integration_info.integration.id))
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
                                href: crate::routes::prompts::MyAssistants { team_id }.to_string(),
                                button_scheme: ButtonScheme::Error,
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
