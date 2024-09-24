#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RemoveInviteDrawer(team_id: i32, invite_id: i32, trigger_id: String) -> Element {
    rsx! {
        Drawer {
            submit_action: crate::routes::team::DeleteInvite{team_id}.to_string(),
            label: "Remove this invite?",
            trigger_id: &trigger_id,
            DrawerBody {
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
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Danger,
                    "Remove Invite"
                }
            }
        }
    }
}
