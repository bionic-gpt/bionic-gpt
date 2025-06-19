#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    SectionIntroduction,
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
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Chat History".into(),
                        href: None
                    }]
                }
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
            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: "Chat History".to_string(),
                    subtitle: "Easily reference past conversations to recall information, follow up on topics, or continue where you left off.".to_string(),
                    is_empty: buckets.1 == 0,
                    empty_text: "You haven't had any conversations yet. When you do, a summary will appear on this page.".to_string(),
                }

                if buckets.1 > 0 {
                    super::history_table::HistoryTable {
                        team_id,
                        buckets: buckets.0
                    }
                }
            }
        }
    };

    crate::render(page)
}
