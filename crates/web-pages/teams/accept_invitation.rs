#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn AcceptInvite(invite: db::Invitation) -> Element {
    rsx! {
        // The form to create an invitation
        form {
            method: "post",
            action: "",
            Drawer {
                label: "Do you want to accept this invitation?",
                trigger_id: format!("accept-invite-trigger-{}", invite.id),
                DrawerBody {
                    div {
                        class: "flex flex-col",
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
