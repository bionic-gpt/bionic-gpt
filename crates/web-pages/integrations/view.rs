#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;

use super::IntegrationOas3;

pub fn view(team_id: i32, rbac: Rbac, integration: Option<IntegrationOas3>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4 max-w-3xl w-full mx-auto",
            selected_item: SideBar::Integrations,
            team_id: team_id,
            rbac: rbac,
            title: "Integrations",
            header: rsx!(
                h3 { "Integration" }
            ),

            if let Some(integration) = integration {
                div {
                    class: "flex",
                    img {
                        class: "border rounded p-2",
                        src: super::get_logo_url(&integration.spec.info.extensions),
                        width: "48",
                        height: "48"
                    }
                    div {
                        class: "ml-4",
                        h2 {
                            class: "text-xl font-semibold",
                            "{integration.spec.info.title.clone()}"
                        }
                        p {
                            if let Some(description) = integration.spec.info.description.clone() {
                                "{description}"
                            }
                        }
                    }
                }
                hr {
                    class: "mt-5 mb-5"
                }
                h2 {
                    class: "font-semibold",
                    "Actions"
                }
                if let Some(map) = integration.spec.paths {
                    for (_path, item) in map {

                        details { class: "card mt-5",
                            summary {
                                class: "cursor-pointer px-4 py-3 flex items-center justify-between",
                                div {
                                    class: "flex",
                                    div {
                                        class: "pt-3",
                                        img {
                                            class: "border rounded p-1",
                                            src: super::get_logo_url(&integration.spec.info.extensions),
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        h2 {
                                            class: "font-semibold",
                                            if let Some(summary) = &item.summary {
                                                "{summary}"
                                            }

                                            if let Some(get) = &item.get {
                                                if let Some(summary) = &get.summary {
                                                    "{summary}"
                                                }
                                            }

                                            if let Some(put) = &item.put {
                                                if let Some(summary) = &put.summary {
                                                    "{summary}"
                                                }
                                            }
                                        }
                                        p {
                                            if let Some(description) = integration.spec.info.description.clone() {
                                                "{description}"
                                            }
                                        }
                                    }
                                }
                            }
                            "Creates a new record in a table"
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
