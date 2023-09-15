#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps {
    organisation_id: i32,
    document_id: i32,
    dataset_id: i32,
    trigger_id: String,
}

pub fn DeleteDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::documents::delete_route(cx.props.organisation_id, cx.props.document_id),
            label: "Delete this document?",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                div {
                    class: "d-flex flex-column",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "Are you sure you want to delete this document?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "organisation_id",
                        "value": "{cx.props.organisation_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "document_id",
                        "value": "{cx.props.document_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "dataset_id",
                        "value": "{cx.props.dataset_id}"
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Delete Document"
                }
            }
        }
    })
}
