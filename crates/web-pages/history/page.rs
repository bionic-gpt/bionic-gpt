#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    i18n, SectionIntroduction,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, History};
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: String, history: Vec<History>, locale: &str) -> String {
    let buckets = super::bucket_history(history);
    let history_label = i18n::histories(locale);
    let history_single = i18n::history(locale);
    let search_label = format!("Search {}", history_single);
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::History,
            team_id: team_id.clone(),
            rbac: rbac,
            title: history_label.clone(),
            locale: Some(locale.to_string()),
            header: rsx! {
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: history_label.clone(),
                        href: None
                    }]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "{search_label}"
                }
            },
            super::form::Form {
                team_id: team_id.clone()
            }
            div {
                class: "p-4 max-w-3xl w-full mx-auto",
                SectionIntroduction {
                    header: history_label.clone(),
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
