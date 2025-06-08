#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    hero::Hero,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, History};
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
            header: rsx! {
                h3 { "Chat History" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search History"
                }
            },
            super::form::Form {
                team_id: team_id
            }
            if buckets.1 == 0 {
                BlankSlate {
                    heading: "Looks like you haven't had any conversations yet",
                    visual: nav_history_svg.name,
                    description: "When you do a summary will appear on this page"
                }
            } else {

                Hero {
                    heading: "Chat History".to_string(),
                    subheading: "Easily reference past conversations to recall information,
                        follow up on topics, or continue where you left off.".to_string()
                }

                super::history_table::HistoryTable {
                    team_id,
                    buckets: buckets.0
                }
            }
        }
    };

    crate::render(page)
}
