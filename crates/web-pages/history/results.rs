#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::nav_history_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::History;
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32, history: Vec<History>) -> String {
    let buckets = super::bucket_history(history);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: "Chat History",
            header: rsx!(
                h3 { "Chat History" }
                Button {
                    popover_target: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search Chats"
                }
            ),
            if buckets.1 == 0 {
                BlankSlate {
                    heading: "We didn't find any results for your search",
                    visual: nav_history_svg.name,
                    description: "Please try agian with a different query"
                }
            } else {
                super::history_table::HistoryTable {
                    team_id,
                    buckets: buckets.0
                }
            }

            // Drawers have to be fairly high up in the hierarchy or they
            // get missed off in turbo::load
            super::form::Form {
                team_id: team_id
            }
        }
    };

    crate::render(page)
}
