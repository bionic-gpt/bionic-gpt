#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
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

#[derive(Debug, Clone)]
pub struct IntegrationWithAuthInfo {
    pub integration: Integration,
    pub requires_api_key: bool,
    pub requires_oauth2: bool,
    pub has_connections: bool,
}

#[derive(Debug, Clone)]
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

                form {
                    action: crate::routes::prompts::UpdateIntegrations { team_id, prompt_id: form.prompt_id }.to_string(),
                    method: "post",
                    class: "space-y-6",

                    // Display error if present
                    if let Some(error) = &form.error {
                        div {
                            class: "alert alert-error mb-4",
                            "{error}"
                        }
                    }

                    // Hidden prompt ID field
                    input {
                        "type": "hidden",
                        value: "{form.prompt_id}",
                        name: "prompt_id"
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
                                "Select which integrations this assistant can use"
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
                                                th { "Add?" }
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
                                                        div {
                                                            class: "flex items-center gap-2",
                                                            if form.selected_integration_ids.contains(&integration_info.integration.id) {
                                                                CheckBox {
                                                                    checked: true,
                                                                    name: "integrations",
                                                                    value: "{integration_info.integration.id}"
                                                                }
                                                            } else {
                                                                CheckBox {
                                                                    name: "integrations",
                                                                    value: "{integration_info.integration.id}",
                                                                    //disabled: (integration_info.requires_api_key || integration_info.requires_oauth2) && !integration_info.has_connections
                                                                }
                                                            }

                                                            // Connection configuration button
                                                            if (integration_info.requires_api_key || integration_info.requires_oauth2) && integration_info.has_connections {
                                                                Button {
                                                                    button_type: ButtonType::Button,
                                                                    button_size: ButtonSize::Small,
                                                                    popover_target: format!("connection-modal-{}", integration_info.integration.id),
                                                                    disabled: !form.selected_integration_ids.contains(&integration_info.integration.id),
                                                                    "Configure"
                                                                }
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

                    // Form Actions
                    Card {
                        CardBody {
                            div {
                                class: "flex justify-between",
                                Button {
                                    button_type: ButtonType::Link,
                                    href: crate::routes::prompts::View { team_id, prompt_id: form.prompt_id }.to_string(),
                                    button_scheme: ButtonScheme::Error,
                                    "Cancel"
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Primary,
                                    "Save Integration Connections"
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
