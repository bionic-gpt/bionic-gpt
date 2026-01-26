#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug, Clone)]
pub struct ProviderForm {
    pub id: Option<i32>,
    pub name: String,
    pub svg_logo: String,
    pub default_model_name: String,
    pub default_model_display_name: String,
    pub default_model_context_size: i32,
    pub default_model_description: String,
    pub base_url: String,
    pub api_key_optional: bool,
    pub default_embeddings_model_name: String,
    pub default_embeddings_model_display_name: String,
    pub default_embeddings_model_context_size: i32,
    pub default_embeddings_model_description: String,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: String, rbac: Rbac, form: ProviderForm) -> String {
    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::Providers,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            title: "Providers",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Providers".into(),
                            href: Some(crate::routes::providers::Index{team_id: team_id.clone()}.to_string())
                        },
                        BreadcrumbItem {
                            text: if form.id.is_some() { "Edit Provider".into() } else { "New Provider".into() },
                            href: None
                        }
                    ]
                }
                h3 {
                    if form.id.is_some() { "Edit Provider" } else { "Create Provider" }
                }
            ),
            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                form {
                    action: crate::routes::providers::Upsert { team_id: team_id.clone() }.to_string(),
                    method: "post",
                    class: "space-y-6",
                    if let Some(id) = form.id {
                        input {
                            "type": "hidden",
                            value: "{id}",
                            name: "id"
                        }
                    }

                    Card {
                        class: "mb-6",
                        CardHeader { title: "Provider Details" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Provider Name",
                                        legend_class: "mt-4",
                                        help_text: "A short name for this model provider.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "name",
                                            value: form.name.clone(),
                                            required: true
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Base URL",
                                        legend_class: "mt-4",
                                        help_text: "The base URL for the provider API.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "base_url",
                                            value: form.base_url.clone(),
                                            required: true
                                        }
                                    }
                                }
                            }
                            div {
                                class: "flex flex-col",
                                Fieldset {
                                    legend: "SVG Logo",
                                    legend_class: "mt-4",
                                    help_text: "Paste the SVG markup for the provider logo.",
                                    TextArea {
                                        class: "mt-3 w-full",
                                        name: "svg_logo",
                                        rows: "8",
                                        required: true,
                                        "{form.svg_logo}"
                                    }
                                }
                            }
                        }
                    }

                    Card {
                        class: "mb-6",
                        CardHeader { title: "Default Model" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Default Model Name",
                                        legend_class: "mt-4",
                                        help_text: "The model identifier used by the provider API.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "default_model_name",
                                            value: form.default_model_name.clone()
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Default Model Display Name",
                                        legend_class: "mt-4",
                                        help_text: "A friendly name shown in the UI.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "default_model_display_name",
                                            value: form.default_model_display_name.clone()
                                        }
                                    }
                                }
                            }
                            Fieldset {
                                legend: "Default Model Context Size",
                                legend_class: "mt-4",
                                help_text: "Context window size for the default model.",
                                Input {
                                    input_type: InputType::Number,
                                    name: "default_model_context_size",
                                    value: "{form.default_model_context_size}",
                                    required: true
                                }
                            }
                            Fieldset {
                                legend: "Default Model Description",
                                legend_class: "mt-4",
                                help_text: "A short description of the default model.",
                                TextArea {
                                    class: "mt-3 w-full",
                                    name: "default_model_description",
                                    rows: "6",
                                    required: true,
                                    "{form.default_model_description}"
                                }
                            }
                            div {
                                class: "form-control mt-4",
                                label {
                                    class: "label cursor-pointer justify-start gap-3",
                                    input {
                                        "type": "checkbox",
                                        name: "api_key_optional",
                                        class: "checkbox",
                                        checked: form.api_key_optional
                                    }
                                    span { class: "label-text", "API key is optional for this provider" }
                                }
                            }
                        }
                    }

                    Card {
                        class: "mb-6",
                        CardHeader { title: "Default Embeddings Model" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Default Embeddings Model Name",
                                        legend_class: "mt-4",
                                        help_text: "Optional identifier for embeddings models.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "default_embeddings_model_name",
                                            value: form.default_embeddings_model_name.clone()
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Default Embeddings Display Name",
                                        legend_class: "mt-4",
                                        help_text: "Optional friendly name for the embeddings model.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "default_embeddings_model_display_name",
                                            value: form.default_embeddings_model_display_name.clone()
                                        }
                                    }
                                }
                            }
                            Fieldset {
                                legend: "Default Embeddings Context Size",
                                legend_class: "mt-4",
                                help_text: "Optional context window size for embeddings.",
                                Input {
                                    input_type: InputType::Number,
                                    name: "default_embeddings_model_context_size",
                                    value: "{form.default_embeddings_model_context_size}",
                                    required: false
                                }
                            }
                            Fieldset {
                                legend: "Default Embeddings Description",
                                legend_class: "mt-4",
                                help_text: "Optional description for the embeddings model.",
                                TextArea {
                                    class: "mt-3 w-full",
                                    name: "default_embeddings_model_description",
                                    rows: "6",
                                    required: false,
                                    "{form.default_embeddings_model_description}"
                                }
                            }
                        }
                    }

                    div {
                        class: "flex justify-between mt-4",
                        Button {
                            button_type: ButtonType::Link,
                            href: crate::routes::providers::Index { team_id: team_id.clone() }.to_string(),
                            button_scheme: ButtonScheme::Error,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            if form.id.is_some() { "Update Provider" } else { "Create Provider" }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
