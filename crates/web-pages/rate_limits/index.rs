#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, RateLimit};
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, rate_limits: Vec<RateLimit>) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::RateLimits,
            team_id: team_id,
            rbac: rbac,
            title: "Rate Limits",
            header: rsx! {
                h3 { "Rate Limits (Trial)" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-limit",
                    button_scheme: ButtonScheme::Primary,
                    "Add Limit"
                }
            },
            BlankSlate {
                heading: "Bionic can assign token limts based on a users role.",
                visual: limits_svg.name,
                description: "Roles are assigned in your identity system and mapped here to limits"
            }

            super::RateTable { rate_limits }

            // Our pop out drawer to add limits
            super::form::Form {
                team_id: team_id
            }
        }
    }
}
