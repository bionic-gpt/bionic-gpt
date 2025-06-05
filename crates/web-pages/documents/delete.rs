#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteDrawer(
    team_id: i32,
    document_id: i32,
    dataset_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        form {
            action: crate::routes::documents::Delete{team_id, document_id}.to_string(),
            method: "post",
            Modal {
                trigger_id: trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Delete this document?"
                    }
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
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "document_id",
                            "value": "{document_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "dataset_id",
                            "value": "{dataset_id}"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Danger,
                            "Delete Document"
                        }
                    }
                }
            }
        }
    }
}
