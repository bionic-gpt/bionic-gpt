#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::queries::{datasets::Dataset, documents::Document};
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    organisation_id: i32,
    dataset: Dataset,
    documents: Vec<Document>,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Datasets,
            team_id: *organisation_id,
            title: "{dataset.name} / Documents",
            header: cx.render(rsx!(
                h3 { "{dataset.name} / Documents" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "upload-form",
                    button_scheme: ButtonScheme::Primary,
                    "Add Document"
                }
            )),

            if documents.is_empty() {
                cx.render(rsx! {
                    BlankSlate {
                        heading: "Looks like this dataset doesn't have any documents yet",
                        visual: nav_ccsds_data_svg.name,
                        description: "Here you can upload documents in a range of formats",
                        primary_action_drawer: (
                            "Add a Document", 
                            "upload-form"
                        )
                    }
                })
            } else {
                cx.render(rsx! {
                    Box {
                        class: "has-data-table",
                        BoxHeader {
                            title: "Documents"
                        }
                        BoxBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "No. Chunks" }
                                    th { "Content Size (Bytes)" }
                                    th { "Status" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {

                                    documents.iter().map(|doc| {
                                        cx.render(rsx!(
                                            tr {
                                                td { "{doc.file_name}" }
                                                td { "{doc.batches}" }
                                                td { "{doc.content_size}" }
                                                td {
                                                    if doc.waiting > 0 {
                                                        cx.render(rsx!(
                                                            turbo-frame {
                                                                id: "status-{doc.id}",
                                                                loading: "lazy",
                                                                src: "{super::super::routes::documents::status_route(doc.id)}",
                                                                Label {
                                                                    "Processing ({doc.waiting} remaining)"
                                                                }
                                                            }
                                                        ))
                                                    } else if doc.batches == 0 {
                                                        cx.render(rsx!(
                                                            turbo-frame {
                                                                id: "status-{doc.id}",
                                                                loading: "lazy",
                                                                src: "{super::super::routes::documents::status_route(doc.id)}",
                                                                Label {
                                                                    "Queued"
                                                                }
                                                            }
                                                        ))
                                                    } else if doc.fail_count > 0 {
                                                        cx.render(rsx!(
                                                            Label {
                                                                label_role: LabelRole::Danger,
                                                                "Processed ({doc.fail_count} failed)"
                                                            }
                                                        ))
                                                    } else {
                                                        cx.render(rsx!(
                                                            Label {
                                                                label_role: LabelRole::Success,
                                                                "Processed"
                                                            }
                                                        ))
                                                    }
                                                }
                                                td {
                                                    class: "text-right",
                                                    DropDown {
                                                        direction: Direction::Left,
                                                        button_text: "...",
                                                        DropDownLink {
                                                            drawer_trigger: format!("delete-doc-trigger-{}-{}", 
                                                                doc.id, *organisation_id),
                                                            href: "#",
                                                            target: "_top",
                                                            "Delete Document"
                                                        }
                                                    }
                                                }
                                            }
                                        ))
                                    })
                                }
                            }
                        }
                    }
                })
            }
        }


        documents.iter().map(|doc| rsx!(
            cx.render(rsx!(
                super::delete::DeleteDrawer {
                    organisation_id: *organisation_id,
                    document_id: doc.id,
                    dataset_id: doc.dataset_id,
                    trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, *organisation_id)
                }
            ))
        ))


        // The form to create an invitation
        super::upload::Upload {
            upload_action: crate::routes::documents::upload_route(*organisation_id, dataset.id)
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
