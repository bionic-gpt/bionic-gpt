#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteDrawer(team_id: i32, id: i64, trigger_id: String) -> Element {
    rsx! {
        form {
            action: crate::routes::console::Delete{team_id, id}.to_string(),
            method: "post",
            Modal {
                trigger_id: trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Delete this Conversation?"
                    }
                    div {
                        class: "flex flex-col",
                        Alert {
                            alert_color: AlertColor::Warn,
                            class: "mb-3",
                            p {
                                "Are you sure you want to delete this Conversation?"
                            }
                        }
                        input {
                            "type": "hidden",
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "id",
                            "value": "{id}"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Danger,
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
