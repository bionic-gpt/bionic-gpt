#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RemoveMemberDrawer(
    team_id: i32,
    email: String,
    user_id: i32,
    trigger_id: String,
) -> Element {
    rsx! {
        form {
            action: crate::routes::team::Delete{team_id}.to_string(),
            method: "post",
            Modal {
                trigger_id: &trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Remove this user?"
                    }
                    div {
                        class: "flex flex-col",
                        Alert {
                            alert_color: AlertColor::Warn,
                            class: "mb-3",
                            h4 {
                                "Are you sure you want to remove '{email}' from the team?"
                            }
                        }
                        input {
                            "type": "hidden",
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "user_id",
                            "value": "{user_id}"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Danger,
                            "Remove User"
                        }
                    }
                }
            }
        }
    }
}
