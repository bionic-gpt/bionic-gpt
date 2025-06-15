#![allow(non_snake_case)]
use daisy_rsx::*;
use db::InviteSummary;
use dioxus::prelude::*;

#[component]
pub fn InviteCard(invite: InviteSummary) -> Element {
    rsx!(
        Card {
            class: "p-3 flex flex-row justify-between",
            div {
                class: "flex flex-row items-center",
                Avatar {
                    avatar_type: avatar::AvatarType::User,
                    name: "{invite.first_name}"
                }
                div {
                    class: "ml-4 flex flex-col",
                    h2 {
                        class: "font-semibold text-base mb-1",
                        "{invite.team_name}"
                    }
                    p {
                        class: "text-sm text-base-content/70",
                        "Invited by {invite.created_by}"
                    }
                }
            }
            div {
                class: "flex flex-col justify-center ml-4",
                Button {
                    button_scheme: ButtonScheme::Primary,
                    button_size: ButtonSize::Small,
                    popover_target: format!("accept-invite-trigger-{}", invite.id),
                    "Accept Invite"
                }
            }
        }
    )
}
