#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::HistoryResult;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, results: Vec<HistoryResult>) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::History,
            team_id: team_id,
            rbac: rbac,
            title: "Chat History",
            header: rsx!(
                h3 { "Chat History" }
                Button {
                    drawer_trigger: "search-history",
                    button_scheme: ButtonScheme::Primary,
                    "Search Chats"
                }
            ),
            h2 {
                class: "mb-4",
                "Search Results..."
            }
            div {
                class: "grid md:grid-cols-3 xl:grid-cols-4 sm:grid-cols-1 gap-4",
                for result in results {
                    Box {
                        BoxHeader {
                            class: "truncate ellipses",
                            title: "I have this rust code. let (request, model_id, user_id) =
                            create_request(&pool, &current_user, chat_id).await?; 
                            How do I check for an error before proceeding "
                        }
                        BoxBody {
                            a {
                                "{result.summary}"
                            }
                        }
                    }
                }
            }

            // Drawers have to be fairly high up in the hierarchy or they
            // get missed off in turbo::load
            super::form::Form {
                team_id: team_id
            }
        }
    }
}
