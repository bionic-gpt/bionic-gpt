#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::integrations::Integration;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, integrations: Vec<Integration>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integrations" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-integration-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Integration"
                }
            ),

            super::integration_table::IntegrationTable {
                integrations: integrations.clone(),
                team_id: team_id
            }

            // The form to create a model
            super::form::Form {
                team_id: team_id,
                trigger_id: "new-inteegration-form".to_string(),
                name: "".to_string(),
                integration_type: "MCP Server".to_string(),
                base_url: "".to_string(),
            }

            for integration in &integrations {
                super::form::Form {
                    id: integration.id,
                    team_id: team_id,
                    trigger_id: format!("edit-integration-form-{}", integration.id),
                    name: integration.name.clone(),
                    base_url: integration.base_url.clone(),
                    integration_type: super::integration_type(integration.integration_type),
                }
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
