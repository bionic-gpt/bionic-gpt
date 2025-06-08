#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, documents::Document};
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32, dataset: Dataset, documents: Vec<Document>) -> String {
    let page = rsx! {
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
                    popover_target: "upload-form",
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
                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "Documents"
                    }
                    CardBody {
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
                    ConfirmModal {
                        action: crate::routes::documents::Delete{team_id, document_id: doc.id}.to_string(),
                        trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, team_id),
                        submit_label: "Delete Document".to_string(),
                        heading: "Delete this document?".to_string(),
                        warning: "Are you sure you want to delete this document?".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), team_id.to_string()),
                            ("document_id".into(), doc.id.to_string()),
                            ("dataset_id".into(), doc.dataset_id.to_string()),
                        ],
                    }
                }

                // The form to create an invitation
                super::upload::Upload {
                    upload_action: crate::routes::documents::Upload{team_id, dataset_id: dataset.id}.to_string()
                }
            }
        }
    };

    crate::render(page)
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
                        popover_target: format!("delete-doc-trigger-{}-{}",
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
