#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::Integration;
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
    pub integrations: Vec<Integration>,
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
                                                th { "Add?" }
                                            }
                                        }
                                        tbody {
                                            for integration in &form.integrations {
                                                tr {
                                                    td { "{integration.name}" }
                                                    td { "{integration.integration_type:?}" }
                                                    td {
                                                        span {
                                                            class: match integration.integration_status {
                                                                db::IntegrationStatus::Configured => "badge badge-success",
                                                                _ => "badge badge-warning"
                                                            },
                                                            "{integration.integration_status:?}"
                                                        }
                                                    }
                                                    td {
                                                        if form.selected_integration_ids.contains(&integration.id) {
                                                            CheckBox {
                                                                checked: true,
                                                                name: "integrations",
                                                                value: "{integration.id}"
                                                            }
                                                        } else {
                                                            CheckBox {
                                                                name: "integrations",
                                                                value: "{integration.id}"
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
