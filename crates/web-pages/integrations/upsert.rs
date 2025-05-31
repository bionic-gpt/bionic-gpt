#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct IntegrationForm {
    pub id: Option<i32>,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "OpenAPI specification is required"))]
    pub openapi_spec: String,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, integration: IntegrationForm) -> String {
    let placeholder= "{\n  \"openapi\": \"3.0.0\",\n  \"info\": {\n    \"title\": \"Your API\",\n    \"version\": \"1.0.0\"\n  },\n  \"paths\": {}\n}";

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integrations" }
            ),

            Card {
                CardHeader {
                    title: "Integrations"
                }
                CardBody {
                    form {
                        method: "post",
                        class: "flex flex-col",
                        if let Some(id) = integration.id {
                            input {
                                "type": "hidden",
                                value: "{id}",
                                name: "id"
                            }
                        }

                        // Display error if present
                        if let Some(error) = &integration.error {
                            div {
                                class: "alert alert-error",
                                "{error}"
                            }
                        }

                        Input {
                            input_type: InputType::Text,
                            label_class: "mt-4",
                            name: "name",
                            label: "Name",
                            help_text: "Make the name memorable and imply it's usage.",
                            value: integration.name
                        }

                        div {
                            class: "mt-4",
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "OpenAPI Specification (JSON)"
                            }
                            TextArea {
                                class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm font-mono leading-tight overflow-y-auto",
                                name: "openapi_spec",
                                rows: "20",
                                placeholder,
                                "{integration.openapi_spec}"
                            }
                            p {
                                class: "mt-1 text-sm text-gray-500",
                                "Paste your complete OpenAPI 3.0+ specification in JSON format"
                            }
                        }

                        div {
                            class: "mt-5 flex justify-between",
                            crate::button::Button {
                                button_type: crate::button::ButtonType::Link,
                                href: crate::routes::integrations::Index { team_id }.to_string(),
                                button_scheme: crate::button::ButtonScheme::Danger,
                                "Cancel"
                            }
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Submit"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
