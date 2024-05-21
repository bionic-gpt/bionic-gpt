#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Invitation;
use dioxus::prelude::*;

#[component]
pub fn InvitationView(invite: Invitation) -> Element {
    rsx! {
        // The form to create an invitation
        Drawer {
            label: "Invite people into your team.",
            trigger_id: format!("invitation-view-trigger-{}", invite.id),
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Success,
                        class: "mt-4 flex flex-col items-start",
                        label {
                            input {
                                "type": "checkbox",
                                name: "admin"
                            }
                            strong {
                                class: "ml-2",
                                "Invite as Team Manager"
                            }
                        }
                        p {
                            class: "note",
                            "Team Managers can invite new team members"
                        }
                    }
                }
            }
            DrawerFooter {
            }
        }
    }
}
