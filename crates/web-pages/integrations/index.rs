#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, integrations: Vec<super::IntegrationOas3>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Integrations",
            header: rsx!(
                h3 { "Integrations" }
                if rbac.can_manage_integrations() {
                    crate::button::Button {
                        button_type: crate::button::ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::integrations::Upsert{team_id}.to_string(),
                        button_scheme: crate::button::ButtonScheme::Primary,
                        "Add Integration"
                    }
                }
            ),

            super::integration_cards::IntegrationCards {
                integrations: integrations.clone(),
                team_id: team_id
            }

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
