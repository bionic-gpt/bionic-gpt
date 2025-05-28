#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use dioxus::prelude::*;
use oas3::spec::PathItem;

use super::IntegrationOas3;

fn extract_summaries_and_descriptions(item: &PathItem) -> Vec<(String, String, String)> {
    [&item.get, &item.put, &item.post, &item.delete]
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|op| {
            let summary = op.summary.clone().unwrap_or_default();
            let description = op.description.clone().unwrap_or_default();
            let operationId = op.operation_id.clone().unwrap_or_default();
            (summary, description, operationId)
        })
        .collect()
}

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
                                        class: "",
                                        img {
                                            class: "border rounded p-1",
                                            src: super::get_logo_url(&integration.spec.info.extensions),
                                            width: "24",
                                            height: "24"
                                        }
                                    }
                                    div {
                                        class: "ml-4",
                                        for (summary, description, operationId) in extract_summaries_and_descriptions(&item) {
                                            h2 {
                                                class: "font-semibold",
                                                "{operationId}"
                                            }
                                            p {
                                                "{description}{summary}"
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
