#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Invitation;
use dioxus::prelude::*;

use crate::routes::team::AcceptInvite;

#[component]
pub fn InvitationView(invite: Invitation) -> Element {
    let url = AcceptInvite {
        invite_selector: invite.invitation_selector,
        invite_validator: invite.invitation_verifier_hash,
    }
    .to_string();

    rsx! {
        // The form to create an invitation
        Drawer {
            label: "View Invitation",
            trigger_id: format!("invitation-view-trigger-{}", invite.id),
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Success,
                        class: "mt-4 flex flex-col items-start",
                        p {
                            "Team Managers can invite new team members"
                        }
                    }
                    TextArea {
                        class: "mt-8",
                        name: "invite",
                        rows: "10",
                        "{url}"
                    }
                }
            }
            DrawerFooter {
            }
        }
    }
}
