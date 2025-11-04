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

pub fn page(team_id: i32, rbac: Rbac, form: DatasetForm, locale: &str) -> String {
    let assistants_label = crate::i18n::assistants(locale);
    let assistant_label = crate::i18n::assistant(locale);
    let datasets_label = crate::i18n::datasets(locale);
    let dataset_label = crate::i18n::dataset(locale);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: format!("Manage {}", datasets_label.clone()),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: assistants_label.clone(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: format!("My {}", assistants_label.clone()),
                            href: Some(crate::routes::prompts::MyAssistants{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: format!("Manage {}", datasets_label.clone()),
                            href: None
                        }
                    ]
                }
                h3 {
                    {format!("Manage {} for {}", datasets_label.clone(), form.prompt_name)}
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
                            title: format!("Available {}", datasets_label.clone())
                        }
                        CardBody {
                            Alert {
                                class: "mb-4",
                                {format!(
                                    "Select which {} you wish to attach to this {}",
                                    datasets_label.clone(),
                                    assistant_label.to_lowercase()
                                )}
                            }

                            if !form.datasets.is_empty() {
                                div {
                                    class: "overflow-x-auto",
                                    table {
                                        class: "table table-sm w-full",
                                        thead {
                                            tr {
                                                th { "{dataset_label}" }
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
                                    {format!("No {} available", datasets_label)}
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
                                    href: crate::routes::prompts::MyAssistants { team_id }.to_string(),
                                    button_scheme: ButtonScheme::Error,
                                    "Cancel"
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Primary,
                                    {format!("Save {} Connections", datasets_label)}
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
