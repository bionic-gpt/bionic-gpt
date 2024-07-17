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
            h2 {
                class: "mb-4",
                "Search Results..."
            }
            div {
                class: "grid md:grid-cols-3 xl:grid-cols-4 sm:grid-cols-1 gap-4",
                Box {
                    BoxHeader {
                        class: "truncate ellipses",
                        title: "I have this rust code. let (request, model_id, user_id) =
                        create_request(&pool, &current_user, chat_id).await?; 
                        How do I check for an error before proceeding "
                    }
                    BoxBody {
                        a {
                            "...Note that in both examples, you need to handle the error
                            explicitly by returning an error from the current function or 
                            by logging the error and continuing execution..."
                        }
                    }
                },
                Box {
                    BoxHeader {
                        class: "truncate ellipses",
                        title: "What organ nisation enforces SAR requests in munich "
                    }
                    BoxBody {
                        a {
                            "...The BayLDA is the independent data protection authority
                            for the state of Bavaria, where Munich is located. 
                            They are responsible for monitoring and enforcing...."
                        }
                    }
                },
                Box {
                    BoxHeader {
                        class: "truncate ellipses",
                        title: "What organ nisation enforces SAR requests in munich "
                    }
                    BoxBody {
                        a {
                            "...The BayLDA is the independent data protection authority
                            for the state of Bavaria, where Munich is located. 
                            They are responsible for monitoring and enforcing...."
                        }
                    }
                },
                Box {
                    BoxHeader {
                        class: "truncate ellipses",
                        title: "What organ nisation enforces SAR requests in munich "
                    }
                    BoxBody {
                        a {
                            "...The BayLDA is the independent data protection authority
                            for the state of Bavaria, where Munich is located. 
                            They are responsible for monitoring and enforcing...."
                        }
                    }
                },
                Box {
                    BoxHeader {
                        class: "truncate ellipses",
                        title: "What organ nisation enforces SAR requests in munich "
                    }
                    BoxBody {
                        a {
                            "...The BayLDA is the independent data protection authority
                            for the state of Bavaria, where Munich is located. 
                            They are responsible for monitoring and enforcing...."
                        }
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
