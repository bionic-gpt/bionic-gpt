#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn DeleteDrawer(
    cx: Scope<DrawerProps>,
    organisation_id: i32,
    document_id: i32,
    dataset_id: i32,
    trigger_id: String,
) -> Element {
    cx.render(rsx! {
        Drawer {
            submit_action: crate::routes::documents::delete_route(*organisation_id, *document_id),
            label: "Delete this document?",
            trigger_id: trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        p {
                            "Are you sure you want to delete this document?"
                        }
                    }
                    input {
                        "type": "hidden",
                        "name": "organisation_id",
                        "value": "{*organisation_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "document_id",
                        "value": "{*document_id}"
                    }
                    input {
                        "type": "hidden",
                        "name": "dataset_id",
                        "value": "{*dataset_id}"
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
