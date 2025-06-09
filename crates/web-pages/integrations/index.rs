#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::routes;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;
use integrations::BionicOpenAPI;

pub fn page(team_id: i32, rbac: Rbac, integrations: Vec<(BionicOpenAPI, i32)>) -> String {
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
                super::integration_cards::IntegrationCards {
                    integrations,
                    team_id: team_id
                }
            }
        }
    };

    crate::render(page)
}
