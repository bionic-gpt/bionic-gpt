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
    pub base_url: String,
    pub name: String,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, integration: IntegrationForm) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
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

                        Input {
                            input_type: InputType::Text,
                            label_class: "mt-4",
                            name: "base_url",
                            label: "Base Url",
                            help_text: "The base URL of the Open API server",
                            value: integration.base_url
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
