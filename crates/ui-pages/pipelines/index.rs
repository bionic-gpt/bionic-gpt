#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{Dataset, DocumentPipeline};
use dioxus::prelude::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    team_id: i32,
    pipelines: Vec<DocumentPipeline>,
    datasets: Vec<Dataset>,
) -> Element {
    cx.render(rsx! {
        if pipelines.is_empty() {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::DocumentPipelines,
                    team_id: *team_id,
                    title: "Document Pipelines",
                    header: cx.render(rsx!(
                        h3 { "Document Pipelines" }
                    )),
                    BlankSlate {
                        heading: "Automate document upload with our bulk upload API",
                        visual: empty_api_keys_svg.name,
                        description: "The upload API connects your documents to datasets for processing by our pipeline",
                        primary_action_drawer: ("New Document Pipeline", "create-api-key")
                    }

                    super::key_drawer::KeyDrawer {
                        datasets: datasets.clone(),
                        team_id: *team_id,
                    }
                }
            })
        } else {
            cx.render(rsx! {
                Layout {
                    section_class: "normal",
                    selected_item: SideBar::DocumentPipelines,
                    team_id: *team_id,
                    title: "Document Pipelines",
                    header: cx.render(rsx!(
                        h3 { "Document Pipelines" }
                        Button {
                            drawer_trigger: "create-api-key",
                            button_scheme: ButtonScheme::Primary,
                            "New Pipeline"
                        }
                    )),
                    Box {
                        class: "has-data-table",
                        BoxHeader {
                            title: "Document Pipelines"
                        }
                        BoxBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "API Key" }
                                    th { "Dataset" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {
                                    pipelines.iter().map(|key| rsx!(
                                        tr {
                                            td {
                                                "{key.name}"
                                            }
                                            td {
                                                Input {
                                                    value: &key.api_key,
                                                    name: "api_key"
                                                }
                                            }
                                            td {
                                                "{key.dataset_name}"
                                            }
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        href: "#",
                                                        target: "_top",
                                                        "Not Implemented"
                                                    }
                                                }
                                            }
                                        }
                                    ))
                                }
                            }
                        }
                    }

                    super::key_drawer::KeyDrawer {
                        datasets: datasets.clone(),
                        team_id: *team_id,
                    }
                }
            })
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
