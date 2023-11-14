use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::{Dataset, DocumentPipeline};
use dioxus::prelude::*;
use primer_rsx::*;

struct PipelineKeysProps {
    organisation_id: i32,
    pipelines: Vec<DocumentPipeline>,
    datasets: Vec<Dataset>,
}

pub fn index(
    pipelines: Vec<DocumentPipeline>,
    datasets: Vec<Dataset>,
    organisation_id: i32,
) -> String {
    fn app(cx: Scope<PipelineKeysProps>) -> Element {
        cx.render(rsx! {
            if cx.props.pipelines.is_empty() {
                cx.render(rsx! {
                    Layout {
                        section_class: "normal",
                        selected_item: SideBar::DocumentPipelines,
                        team_id: cx.props.organisation_id,
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
                    }
                })
            } else {
                cx.render(rsx! {
                    Layout {
                        section_class: "normal",
                        selected_item: SideBar::DocumentPipelines,
                        team_id: cx.props.organisation_id,
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
                            BoxHeader {
                                title: "Document Pipelines"
                            }
                            BoxBody {
                                DataTable {
                                    table {
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
                                            cx.props.pipelines.iter().map(|key| rsx!(
                                                tr {
                                                    td {
                                                        "{key.name}"
                                                    }
                                                    td {
                                                        Input {
                                                            value: &key.api_key,
                                                            name: "api_key",
                                                            disabled: true
                                                        }
                                                    }
                                                    td {
                                                        "{key.dataset_name}"
                                                    }
                                                    td {
                                                        class: "text-right",
                                                        SelectMenu {
                                                            alignment: SelectMenuAlignment::Right,
                                                            summary: cx.render(rsx!(
                                                                summary {
                                                                    class: "btn",
                                                                    "aria-haspopup": "true",
                                                                    "..."
                                                                }
                                                            )),
                                                            SelectMenuModal {
                                                                SelectMenuList {
                                                                    button {
                                                                        class: "SelectMenu-item",
                                                                        role: "menuitemcheckbox",
                                                                        "Not Implemented"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
            }

            super::key_drawer::KeyDrawer {
                datasets: cx.props.datasets.clone(),
                organisation_id: cx.props.organisation_id,
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        PipelineKeysProps {
            pipelines,
            datasets,
            organisation_id,
        },
    ))
}
