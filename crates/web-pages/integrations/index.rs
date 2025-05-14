#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use db::authz::Rbac;
use db::queries::integrations::Integration;
use dioxus::prelude::*;
use integrations::IntegrationTool;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    integrations: Vec<Integration>,
    tool_integrations: Vec<IntegrationTool>,
) -> String {
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
                tool_integrations,
                team_id: team_id
            }

            for item in &integrations {
                super::delete::DeleteDrawer {
                    team_id: team_id,
                    id: item.id,
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                }
            }
        }
    };

    crate::render(page)
}
