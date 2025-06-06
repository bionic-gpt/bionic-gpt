#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AcceptInvite(invite: db::InviteSummary, team_id: i32) -> Element {
    rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: crate::routes::teams::AcceptInvite{}.to_string(),
            Modal {
                trigger_id: format!("accept-invite-trigger-{}", invite.id),
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Do you want to accept this invitation?"
                    }
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
                    ModalAction {
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
}
