#![allow(non_snake_case)]
use crate::app_layout::{AdminLayout, SideBar};
use crate::routes;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct OpenapiSpecForm {
    pub id: Option<i32>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub logo_url: String,
    #[serde(default)]
    pub category: String,
    #[validate(length(min = 1, message = "OpenAPI specification is required"))]
    pub spec: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(skip)]
    pub error: Option<String>,
}

impl Default for OpenapiSpecForm {
    fn default() -> Self {
        Self {
            id: None,
            slug: String::new(),
            title: String::new(),
            description: String::new(),
            logo_url: String::new(),
            category: "Application".to_string(),
            spec: String::new(),
            is_active: true,
            error: None,
        }
    }
}

pub fn page(team_id: String, rbac: Rbac, form: OpenapiSpecForm) -> String {
    let is_edit = form.id.is_some();
    let header_text = if is_edit {
        "Edit OpenAPI Spec"
    } else {
        "Import OpenAPI Spec"
    };

    let page = rsx! {
        AdminLayout {
            section_class: "p-4",
            selected_item: SideBar::OpenapiSpecs,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            title: "OpenAPI Specs",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "OpenAPI Specs".into(),
                            href: Some(routes::openapi_specs::Index { team_id: team_id.clone() }.to_string()),
                        },
                        BreadcrumbItem {
                            text: header_text.into(),
                            href: None,
                        }
                    ]
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                form {
                    method: "post",
                    action: routes::openapi_specs::Upsert { team_id: team_id.clone() }.to_string(),
                    class: "space-y-6",

                    if let Some(id) = form.id {
                        input {
                            r#type: "hidden",
                            name: "id",
                            value: "{id}",
                        }
                    }

                    if let Some(error) = &form.error {
                        Alert {
                            alert_color: AlertColor::Error,
                            class: "mb-2",
                            "{error}"
                        }
                    }

                    Card {
                        CardHeader { title: "Import Settings" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            p {
                                class: "text-sm opacity-70",
                                "Title, description, logo, slug, and API URL are read from the OpenAPI spec."
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Category",
                                        legend_class: "mt-4",
                                        help_text: "Controls where this spec appears in the admin UI.",
                                        select {
                                            name: "category",
                                            class: "select select-bordered w-full",
                                            value: "{form.category}",
                                            SelectOption {
                                                value: "Application",
                                                selected_value: "{form.category}",
                                                "Application"
                                            },
                                            SelectOption {
                                                value: "WebSearch",
                                                selected_value: "{form.category}",
                                                "Web Search"
                                            },
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Status",
                                        legend_class: "mt-4",
                                        help_text: "Inactive specs are hidden from the team catalog.",
                                        label {
                                            class: "flex items-center gap-2 min-h-12",
                                            input {
                                                r#type: "checkbox",
                                                class: "checkbox",
                                                name: "is_active",
                                                value: "true",
                                                checked: form.is_active,
                                            }
                                            span { "Active" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Card {
                        CardHeader { title: "OpenAPI Spec" }
                        CardBody {
                            Fieldset {
                                legend: "OpenAPI Spec (JSON or YAML) *",
                                legend_class: "mt-4",
                                help_text: "Provide the full OpenAPI 3.x specification. The slug is derived from info.x-bionic-slug, info.bionic-slug, or info.title.",
                                TextArea {
                                    class: "format-json font-mono text-sm leading-tight w-full",
                                    name: "spec",
                                    rows: "20",
                                    placeholder: "{{\n  \"openapi\": \"3.1.0\",\n  \"info\": {{ \"title\": \"Sample\" }}\n}}",
                                    required: true,
                                    "{form.spec}"
                                }
                            }
                        }
                    }

                    div {
                        class: "flex justify-between",
                        Button {
                            button_type: ButtonType::Link,
                            href: routes::openapi_specs::Index { team_id: team_id.clone() }.to_string(),
                            button_scheme: ButtonScheme::Error,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            if is_edit {
                                "Save Changes"
                            } else {
                                "Create Spec"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
