#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn DeleteDrawer(team_id: i32, trigger_id: String) -> Element {
    rsx! {
        form {
            action: crate::routes::teams::Delete {team_id}.to_string(),
            method: "post",
            Modal {
                trigger_id: trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Delete this Team?"
                    }
                    div {
                        class: "flex flex-col",
                        Alert {
                            alert_color: AlertColor::Warn,
                            class: "mb-3",
                            p {
                                "Are you sure you want to delete this Team?"
                            }
                        }
                        input {
                            "type": "hidden",
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                    }
                    ModalAction {
                        button {
                            "data-turbo-frame": "_top",
                            "type": "submit",
                            class: "btn btn-primary btn-sm",
                            "Delete",
                        }
                    }
                }
            }
        }
    }
}
