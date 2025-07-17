#![allow(non_snake_case)]
use super::document_card::DocumentCard;
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
use crate::SectionIntroduction;
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
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: dataset.name,
                            href: None
                        },
                        BreadcrumbItem {
                            text: "Documents".into(),
                            href: None
                        }
                    ]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "upload-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Document"
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                SectionIntroduction {
                    header: "Documents".to_string(),
                    subtitle: "Upload and manage documents in various formats for this dataset.".to_string(),
                    is_empty: documents.is_empty(),
                    empty_text: "This dataset doesn't have any documents yet. Upload your first document to get started.".to_string(),
                }

                if !documents.is_empty() {
                    div {
                        class: "space-y-2",
                        for doc in &documents {
                            DocumentCard { document: doc.clone(), team_id: team_id, first_time: true }
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
                }

                // The form to create an invitation - always available
                super::upload::Upload {
                    upload_action: crate::routes::documents::Upload{team_id, dataset_id: dataset.id}.to_string()
                }
            }
        }
    };

    crate::render(page)
}
