#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
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

            if let Some(integration) = integration {
                input {
                    "type": "hidden",
                    value: "{integration.id}",
                    name: "id"
                }
            }
        }
    };

    crate::render(page)
}
