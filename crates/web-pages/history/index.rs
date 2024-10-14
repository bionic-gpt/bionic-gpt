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
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: "Chat History",
            header: rsx! {
                h3 { "Chat History" }
            },
            BlankSlate {
                heading: "Search your chat history",
                visual: nav_history_svg.name,
                description: "Click the search button and enter your criteria",
                primary_action_drawer: Some(("Search History".to_string(), "search-history".to_string()))
            },
            super::form::Form {
                team_id: team_id
            }
        }
    }
}
