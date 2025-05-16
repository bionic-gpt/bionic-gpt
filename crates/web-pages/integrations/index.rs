#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use integrations::IntegrationTool;

pub fn page(team_id: i32, rbac: Rbac, integrations: Vec<IntegrationTool>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integrations" }
                crate::button::Button {
                    button_type: crate::button::ButtonType::Link,
                    prefix_image_src: "{button_plus_svg.name}",
                    href: routes::integrations::Upsert{team_id}.to_string(),
                    button_scheme: crate::button::ButtonScheme::Primary,
                    "Add Integration"
                }
            ),

            super::integration_table::IntegrationTable {
                integrations: integrations.clone(),
                team_id: team_id
            }

            // Add details modals for tool integrations
            for (index, tool) in integrations.iter().enumerate() {
                super::details::DetailsModal {
                    integration: tool.clone(),
                    trigger_id: format!("show-integration-details-{}", index)
                }
            }
        }
    };

    crate::render(page)
}
