#![allow(non_snake_case)]
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Invitation, Member};
use dioxus::prelude::*;

#[component]
pub fn MemberCard(member: Member, rbac: Rbac) -> Element {
    let name = match (&member.first_name, &member.last_name) {
        (Some(f), Some(l)) => format!("{} {}", f, l),
        _ => member.email.clone(),
    };
    rsx!(
        Card {
            class: "p-3 flex flex-row justify-between",
            div {
                class: "flex flex-row items-center",
                Avatar {
                    avatar_size: AvatarSize::Medium,
                    name: "{name}"
                }
                div {
                    class: "ml-4 flex flex-col",
                    h2 {
                        class: "font-semibold text-base mb-1",
                        "{name}"
                    }
                    Label {
                        label_role: LabelRole::Success,
                        class: "w-fit",
                        "Active"
                    }
                }
            }
            div {
                class: "flex items-center gap-2",
                for role in member.roles.clone() {
                    crate::team::team_role::Role { role }
                }
            }
            if rbac.can_make_invitations() && rbac.email != member.email {
                div {
                    class: "flex flex-col justify-center ml-4",
                    DropDown {
                        direction: Direction::Left,
                        button_text: "...",
                        DropDownLink {
                            popover_target: format!("remove-member-trigger-{}-{}", member.id, member.team_id),
                            href: "#",
                            target: "_top",
                            "Remove User From Team"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn InvitePendingCard(invite: Invitation, rbac: Rbac) -> Element {
    let name = format!("{} {}", invite.first_name, invite.last_name);
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
                        "{name}"
                    }
                    Label {
                        label_role: LabelRole::Highlight,
                        class: "w-fit",
                        "Invite Pending"
                    }
                }
            }
            div {
                class: "flex items-center gap-2",
                for role in invite.roles.clone() {
                    crate::team::team_role::Role { role }
                }
            }
            if rbac.can_make_invitations() {
                div {
                    class: "flex flex-col justify-center ml-4",
                    DropDown {
                        direction: Direction::Left,
                        button_text: "...",
                        DropDownLink {
                            popover_target: format!("remove-invite-trigger-{}-{}", invite.id, invite.team_id),
                            href: "#",
                            target: "_top",
                            "Delete Invite"
                        }
                    }
                }
            }
        }
    )
}
