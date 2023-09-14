use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use db::queries::documents::Document;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    upload_action: String,
    documents: Vec<Document>,
}

pub fn index(organisation_id: i32, upload_action: String, documents: Vec<Document>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Datasets,
                team_id: cx.props.organisation_id,
                title: "Documents",
                header: cx.render(rsx!(
                    h3 { "Documents" }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: "upload-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Document"
                    }
                )),

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
                                                        button_text: "..."
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
            }


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
            upload_action,
            documents,
        },
    ))
}
