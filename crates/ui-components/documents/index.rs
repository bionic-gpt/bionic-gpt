use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::queries::{datasets::Dataset, documents::Document};
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    dataset: Dataset,
    upload_action: String,
    documents: Vec<Document>,
}

pub fn index(
    organisation_id: i32,
    upload_action: String,
    dataset: Dataset,
    documents: Vec<Document>,
) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Datasets,
                team_id: cx.props.organisation_id,
                title: "{cx.props.dataset.name} / Documents",
                header: cx.render(rsx!(
                    h3 { "{cx.props.dataset.name} / Documents" }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: "upload-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Document"
                    }
                )),

                if cx.props.documents.is_empty() {
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
                                DataTable {
                                    table {
                                        thead {
                                            th { "Name" }
                                            th { "No. 1Kb Batches" }
                                            th { "Text Size" }
                                            th { "Status" }
                                            th {
                                                class: "text-right",
                                                "Action"
                                            }
                                        }
                                        tbody {

                                            cx.props.documents.iter().map(|doc| {
                                                cx.render(rsx!(
                                                    tr {
                                                        td { "{doc.file_name}" }
                                                        td { "{doc.batches}" }
                                                        td { "{doc.text_size}" }
                                                        td {
                                                            if doc.waiting > 0 {
                                                                cx.render(rsx!(
                                                                    Label {
                                                                        "Processing ({doc.waiting} remaining)"
                                                                    }
                                                                ))
                                                            } else if doc.fail_count > 0 {
                                                                cx.render(rsx!(
                                                                    Label {
                                                                        label_color: LabelColor::Attention,
                                                                        "Processed ({doc.fail_count} failed)"
                                                                    }
                                                                ))
                                                            } else {
                                                                cx.render(rsx!(
                                                                    Label {
                                                                        label_color: LabelColor::Success,
                                                                        "Processed"
                                                                    }
                                                                ))
                                                            }
                                                        }
                                                        td {
                                                            class: "text-right",
                                                            DropDown {
                                                                direction: Direction::West,
                                                                button_text: "...",
                                                                DropDownLink {
                                                                    drawer_trigger: format!("delete-doc-trigger-{}-{}", 
                                                                        doc.id, cx.props.organisation_id),
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
                        }
                    })
                }
            }


            cx.props.documents.iter().map(|doc| rsx!(
                cx.render(rsx!(
                    super::delete::DeleteDrawer {
                        organisation_id: cx.props.organisation_id,
                        document_id: doc.id,
                        dataset_id: doc.dataset_id,
                        trigger_id: format!("delete-doc-trigger-{}-{}", doc.id, cx.props.organisation_id)
                    }
                ))
            ))


            // The form to create an invitation
            super::upload::Upload {
                upload_action: &cx.props.upload_action
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        Props {
            organisation_id,
            dataset,
            upload_action,
            documents,
        },
    ))
}
