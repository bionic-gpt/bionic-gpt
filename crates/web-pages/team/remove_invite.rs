#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RemoveInviteDrawer(team_id: i32, invite_id: i32, trigger_id: String) -> Element {
    rsx! {
        form {
            action: crate::routes::team::DeleteInvite{team_id}.to_string(),
            method: "post",
            Modal {
                trigger_id: &trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Remove this invite?"
                    }
                    div {
                        class: "flex flex-col",
                        Alert {
                            alert_color: AlertColor::Warn,
                            class: "mb-3",
                            h4 {
                                "Are you sure you want to remove this invite?"
                            }
                        }
                        input {
                            "type": "hidden",
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "invite_id",
                            "value": "{invite_id}"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Danger,
                            "Remove Invite"
                        }
                    }
                }
            }
        }
    }
}
