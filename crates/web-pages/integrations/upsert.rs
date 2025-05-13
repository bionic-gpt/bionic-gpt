#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::integrations::Integration;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, integration: Option<Integration>) -> String {
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
                if let Some(integration) = integration {
                    input {
                        "type": "hidden",
                        value: "{integration.id}",
                        name: "id"
                    }
                }

                Input {
                    input_type: InputType::Text,
                    label_class: "mt-4",
                    name: "name",
                    label: "Base Url",
                    help_text: "Make the name memorable and imply it's usage.",
                    value: ""
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
