#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::Dataset;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct DatasetForm {
    pub prompt_id: i32,
    pub prompt_name: String,
    pub selected_dataset_ids: Vec<i32>,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub datasets: Vec<Dataset>,
}

pub fn page(team_id: i32, rbac: Rbac, form: DatasetForm) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Manage Datasets",
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
                            text: "Manage Datasets".into(),
                            href: None
                        }
                    ]
                }
                h3 {
                    "Manage Datasets for {form.prompt_name}"
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",

                form {
                    action: crate::routes::prompts::UpdateDatasets { team_id, prompt_id: form.prompt_id }.to_string(),
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

                    // Datasets Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Available Datasets"
                        }
                        CardBody {
                            Alert {
                                class: "mb-4",
                                "Select which datasets you wish to attach to this assistant"
                            }

                            if !form.datasets.is_empty() {
                                div {
                                    class: "overflow-x-auto",
                                    table {
                                        class: "table table-sm w-full",
                                        thead {
                                            tr {
                                                th { "Dataset" }
                                                th { "Model" }
                                                th { "Add?" }
                                            }
                                        }
                                        tbody {
                                            for dataset in &form.datasets {
                                                tr {
                                                    td { "{dataset.name}" }
                                                    td { "{dataset.embeddings_model_name}" }
                                                    td {
                                                        if form.selected_dataset_ids.contains(&dataset.id) {
                                                            CheckBox {
                                                                checked: true,
                                                                name: "datasets",
                                                                value: "{dataset.id}"
                                                            }
                                                        } else {
                                                            CheckBox {
                                                                name: "datasets",
                                                                value: "{dataset.id}"
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
                                    "No datasets available"
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
                                    "Save Dataset Connections"
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
