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
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search History"
                }
            },
            super::form::Form {
                team_id: team_id
            }
            super::history_table::HistoryTable {

            }
        }
    }
}
