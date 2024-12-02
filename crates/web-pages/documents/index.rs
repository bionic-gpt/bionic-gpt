#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, documents::Document};
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, dataset: Dataset, documents: Vec<Document>) -> Element {
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac,
            title: "{dataset.name} / Documents",
            header: rsx!(
                h3 { "{dataset.name} / Documents" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "upload-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Document"
                }
            ),

            if documents.is_empty() {
                BlankSlate {
                    heading: "Looks like this dataset doesn't have any documents yet",
                    visual: nav_ccsds_data_svg.name,
                    description: "Here you can upload documents in a range of formats",
                    primary_action_drawer: (
                        "Add a Document".to_string(),
                        "upload-form".to_string()
                    )
                }

                // The form to create an invitation
                super::upload::Upload {
                    upload_action: crate::routes::documents::Upload{team_id, dataset_id: dataset.id}.to_string()
                }
            } else {
                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "Documents"
                    }
                    BoxBody {
                        table {
                            id: "documents",
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th {
                                    class: "max-sm:hidden",
                                    "No. Chunks"
                                }
                                th { "Content Size (Bytes)" }
                                th { "Status" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for doc in &documents {
                                        Row {
                                            document: doc.clone(),
                                            team_id: team_id,
                                            first_time: true
                                        }
                                }
                            }
                        }
                    }
                }

                for doc in documents {
                    super::delete::DeleteDrawer {
                        team_id: team_id,
                        document_id: doc.id,
                        dataset_id: doc.dataset_id,
                        trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, team_id)
                    }
                }

                // The form to create an invitation
                super::upload::Upload {
                    upload_action: crate::routes::documents::Upload{team_id, dataset_id: dataset.id}.to_string()
                }
            }
        }
    }
}

#[component]
pub fn Row(document: Document, team_id: i32, first_time: bool) -> Element {
    let text = if let Some(failure_reason) = document.failure_reason.clone() {
        failure_reason.replace(['{', '"', ':', '}'], " ")
    } else {
        "None".to_string()
    };

    let class = if document.waiting > 0 || document.batches == 0 {
        "processing"
    } else {
        "processing-finished"
    };

    let id = format!("processing-label-{}", document.id);

    let src = if first_time {
        Some(
            crate::routes::documents::Processing {
                team_id,
                document_id: document.id,
            }
            .to_string(),
        )
    } else {
        None
    };

    rsx!(
        tr {
            td { "{document.file_name}" }
            td {
                class: "max-sm:hidden",
                "{document.batches}"
             }
            td { "{document.content_size}" }
            td {
                if document.waiting > 0 || document.batches == 0 {
                    turbo-frame {
                        id,
                        src,
                        Label {
                            class: class,
                            "Processing ({document.waiting} remaining)"
                        }
                    }
                } else if document.failure_reason.is_some() {
                    turbo-frame {
                        id,
                        src,

                        ToolTip {
                            text: "{text}",
                            Label {
                                label_role: LabelRole::Danger,
                                "Failed"
                            }
                        }
                    }
                } else if document.batches == 0 {
                    turbo-frame {
                        id,
                        src,

                        Label {
                            "Queued"
                        }
                    }
                } else if document.fail_count > 0 {
                    turbo-frame {
                        id,
                        src,

                        Label {
                            label_role: LabelRole::Danger,
                            "Processed ({document.fail_count} failed)"
                        }
                    }
                } else if document.failure_reason.is_some() {
                    turbo-frame {
                        id,
                        src,

                        Label {
                            label_role: LabelRole::Danger,
                            "Failed"
                        }
                    }
                } else {
                    turbo-frame {
                        id,
                        src,

                        Label {
                            label_role: LabelRole::Success,
                            "Processed"
                        }
                    }
                }
            }
            td {
                class: "text-right",
                DropDown {
                    direction: Direction::Left,
                    button_text: "...",
                    DropDownLink {
                        drawer_trigger: format!("delete-doc-trigger-{}-{}",
                            document.id, team_id),
                        href: "#",
                        target: "_top",
                        "Delete Document"
                    }
                }
            }
        }
    )
}
