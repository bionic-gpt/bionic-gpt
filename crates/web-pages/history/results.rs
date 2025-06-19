#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::SectionIntroduction;
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
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Chat History".into(),
                        href: None
                    }]
                }
                Button {
                    popover_target: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search Chats"
                }
            ),
            SectionIntroduction {
                header: "Search Results".to_string(),
                subtitle: "Browse through your chat history search results.".to_string(),
                is_empty: buckets.1 == 0,
                empty_text: "We didn't find any results for your search. Please try again with a different query.".to_string(),
            }

            if buckets.1 > 0 {
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
