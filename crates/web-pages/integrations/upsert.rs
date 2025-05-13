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

            form {
                method: "post",
                if let Some(id) = integration.id {
                    input {
                        "type": "hidden",
                        value: "{id}",
                        name: "id"
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
                    help_text: "TRhe base URL of the Open API server",
                    value: integration.base_url
                }

                Button {
                    button_type: ButtonType::Submit,
                    "Submit"
                }
            }
        }
    };

    crate::render(page)
}
