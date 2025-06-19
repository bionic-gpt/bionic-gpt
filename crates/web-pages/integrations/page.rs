#![allow(non_snake_case)]
use super::integration_cards::IntegrationSummary;
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, integrations: Vec<IntegrationSummary>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Integrations",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Integrations".into(),
                        href: Some(crate::routes::integrations::Index { team_id }.to_string())
                    }]
                }
                if rbac.can_manage_integrations() {
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::integrations::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        "Add Integration"
                    }
                }
            ),

            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Integrations".to_string(),
                    subtitle: "Connect external tools to retrieve data, take actions, and more.".to_string(),
                    is_empty: integrations.is_empty(),
                    empty_text: "No integrations have been configured yet. Add your first integration to get started.".to_string(),
                }
                if !integrations.is_empty() {
                    super::integration_cards::IntegrationCards {
                        integrations,
                        team_id: team_id
                    }
                }
            }
        }
    };

    crate::render(page)
}
