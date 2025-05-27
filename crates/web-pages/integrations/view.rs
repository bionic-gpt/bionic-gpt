#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use db::Integration;
use dioxus::prelude::*;

pub fn view(team_id: i32, rbac: Rbac, _integration: Integration) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integration" }
            ),

            // Add details modals for tool integrations
            //for (index, tool) in integrations.iter().enumerate() {
            //    super::details_modal::DetailsModal {
            //        integration: tool.clone(),
            //        trigger_id: format!("show-integration-details-{}", index)
            //    }
            //}
        }
    };

    crate::render(page)
}
