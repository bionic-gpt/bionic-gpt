#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::empty_api_keys_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Dataset, DocumentPipeline};
use dioxus::prelude::*;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    pipelines: Vec<DocumentPipeline>,
    datasets: Vec<Dataset>,
) -> Element {
    rsx! {
        if pipelines.is_empty() {
            Layout {
                section_class: "normal",
                selected_item: SideBar::DocumentPipelines,
                team_id: team_id,
                rbac: rbac,
                title: "Document Pipelines",
                header: rsx!(
                    h3 { "Document Pipelines" }
                ),
                BlankSlate {
                    heading: "Automate document upload with our bulk upload API",
                    visual: empty_api_keys_svg.name,
                    description: "The upload API connects your documents to datasets for processing by our pipeline",
                    primary_action_drawer: Some(("New Document Pipeline".to_string(), "create-api-key".to_string()))
                }

                super::key_drawer::KeyDrawer {
                    datasets: datasets.clone(),
                    team_id: team_id,
                }
            }
        } else {
            Layout {
                section_class: "normal",
                selected_item: SideBar::DocumentPipelines,
                team_id: team_id,
                rbac: rbac,
                title: "Document Pipelines",
                header: rsx!(
                    h3 { "Document Pipelines" }
                    Button {
                        drawer_trigger: "create-api-key",
                        button_scheme: ButtonScheme::Primary,
                        "New Pipeline"
                    }
                ),
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
                                for key in &pipelines {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            Input {
                                                value: key.api_key.clone(),
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
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        key.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                for item in pipelines {
                    super::delete::DeleteDrawer {
                        team_id: team_id,
                        id: item.id,
                        trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                    }
                }

                super::key_drawer::KeyDrawer {
                    datasets: datasets.clone(),
                    team_id: team_id,
                }
            }
        }
    }
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
