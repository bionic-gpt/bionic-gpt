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
            selected_item: SideBar::Licence,
            team_id: team_id,
            rbac: rbac,
            title: "Licence",
            header: rsx! {
                h3 { "Your Licence" }
            },
            BlankSlate {
                heading: "GuardRails",
                visual: bionic_logo_svg.name,
                description: "GuardRails",
                primary_action_drawer: Some(("Coming Summer 2024".to_string(), "create-licence".to_string()))
            }
        }
    }
}
