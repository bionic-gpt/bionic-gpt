#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AcceptInvite(invite: db::Invitation, team_id: i32) -> Element {
    rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: crate::routes::teams::AcceptInvite{}.to_string(),
            Drawer {
                label: "Do you want to accept this invitation?",
                trigger_id: format!("accept-invite-trigger-{}", invite.id),
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        input {
                            "type": "hidden",
                            "name": "team_id",
                            "value": "{team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "new_team_id",
                            "value": "{invite.team_id}"
                        }
                        input {
                            "type": "hidden",
                            "name": "invite_id",
                            "value": "{invite.id}"
                        }
                    }
                }
                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Accept Invitation"
                    }
                }
            }
        }
    }
}
