#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::RateLimits,
            team_id: team_id,
            rbac: rbac,
            title: "Rate Limits",
            header: rsx! {
                h3 { "Rate Limits (Trial)" }
            },
            BlankSlate {
                heading: "Looks like you don't have any API keys",
                visual: empty_api_keys_svg.name,
                description: "API Keys allow you to access our programming interface",
                primary_action_drawer: Some(("New API Key".to_string(), "create-api-key".to_string()))
            }

            super::RateTable {}
        }
    }
}
