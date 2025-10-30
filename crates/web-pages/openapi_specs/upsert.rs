#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct OpenapiSpecForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "Slug is required"))]
    pub slug: String,
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    pub description: String,
    pub logo_url: String,
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
            spec: String::new(),
            is_active: true,
            error: None,
        }
    }
}

pub fn page(team_id: i32, rbac: Rbac, form: OpenapiSpecForm) -> String {
    let is_edit = form.id.is_some();
    let header_text = if is_edit {
        "Edit OpenAPI Spec"
    } else {
        "New OpenAPI Spec"
    };

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::OpenapiSpecs,
            team_id,
            rbac: rbac.clone(),
            title: "OpenAPI Specs",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "OpenAPI Specs".into(),
                            href: Some(routes::openapi_specs::Index { team_id }.to_string()),
                        },
                        BreadcrumbItem {
                            text: header_text.into(),
                            href: None,
                        }
                    ]
                }
            ),

            Card {
                CardHeader { title: header_text }
                CardBody {
                    form {
                        method: "post",
                        action: routes::openapi_specs::Upsert { team_id }.to_string(),
                        class: "flex flex-col space-y-6",

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

                        Fieldset {
                            legend: "Slug",
                            help_text: "A unique identifier for this spec. Lowercase letters, numbers, and hyphens recommended.",
                            Input {
                                input_type: InputType::Text,
                                name: "slug",
                                placeholder: "e.g., google-calendar",
                                value: "{form.slug}",
                                required: true
                            }
                        }

                        Fieldset {
                            legend: "Title",
                            help_text: "Displayed name for teams selecting this spec.",
                            Input {
                                input_type: InputType::Text,
                                name: "title",
                                placeholder: "Google Calendar",
                                value: "{form.title}",
                                required: true
                            }
                        }

                        Fieldset {
                            legend: "Description",
                            help_text: "Optional summary shown in the catalog.",
                            TextArea {
                                name: "description",
                                rows: "3",
                                placeholder: "Briefly describe what this integration does.",
                                "{form.description}"
                            }
                        }

                        Fieldset {
                            legend: "Logo URL",
                            help_text: "Optional data URL or HTTPS URL for the logo shown in the catalog.",
                            Input {
                                input_type: InputType::Text,
                                name: "logo_url",
                                placeholder: "data:image/svg+xml;base64,...",
                                value: "{form.logo_url}"
                            }
                        }

                        Fieldset {
                            legend: "Status",
                            help_text: "Inactive specs are hidden from the team catalog.",
                            label {
                                class: "flex items-center gap-2",
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

                        Fieldset {
                            legend: "OpenAPI Spec (JSON)",
                            help_text: "Provide the full OpenAPI 3.x specification in JSON format.",
                            TextArea {
                                class: "format-json font-mono text-sm leading-tight",
                                name: "spec",
                                rows: "20",
                                placeholder: "{{\n  \"openapi\": \"3.1.0\",\n  \"info\": {{ \"title\": \"Sample\" }}\n}}",
                                "{form.spec}"
                            }
                        }

                        div {
                            class: "flex justify-between",
                            Button {
                                button_type: ButtonType::Link,
                                href: routes::openapi_specs::Index { team_id }.to_string(),
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
        }
    };

    crate::render(page)
}
