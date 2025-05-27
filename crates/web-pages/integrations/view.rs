#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

use super::IntegrationOas3;

pub fn view(team_id: i32, rbac: Rbac, integration: Option<IntegrationOas3>) -> String {
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

            if let Some(integration) = integration {
                img {
                    src: super::get_logo_url(&integration.spec.info.extensions),
                    width: "48",
                    height: "48"
                }
                div {
                    class: "ml-4",
                    h2 {
                        "{integration.spec.info.title.clone()}"
                    }
                    p {
                        if let Some(description) = integration.spec.info.description.clone() {
                            "{description}"
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
