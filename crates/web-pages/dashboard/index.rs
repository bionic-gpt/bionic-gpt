#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Dashboard,
            team_id: team_id,
            rbac: rbac,
            title: "Dashboard",
            header: rsx! {
                h3 { "Dashboard" }
            }
        }
    }
}
