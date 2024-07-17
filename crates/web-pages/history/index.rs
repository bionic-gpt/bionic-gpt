#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32) -> Element {
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
                    drawer_trigger: "create-api-key",
                    button_scheme: ButtonScheme::Primary,
                    "Search Chats"
                }
            ),
            div {
                class: "grid md:grid-cols-3 xl:grid-cols-4 sm:grid-cols-1 gap-4",
                Box {
                    BoxHeader {
                        title: "Say Hi"
                    }
                    BoxBody {
                    }
                },
                Box {
                    BoxHeader {
                        title: "Say Hi"
                    }
                    BoxBody {
                    }
                },
                Box {
                    BoxHeader {
                        title: "Say Hi"
                    }
                    BoxBody {
                    }
                },
                Box {
                    BoxHeader {
                        title: "Say Hi"
                    }
                    BoxBody {
                    }
                },
                Box {
                    BoxHeader {
                        title: "Say Hi"
                    }
                    BoxBody {
                    }
                },
            }

            // Drawers have to be fairly high up in the hierarchy or they
            // get missed off in turbo::load
            super::form::Form {
                team_id: team_id
            }
        }
    }
}
