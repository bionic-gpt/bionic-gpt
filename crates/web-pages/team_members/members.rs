#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Invitation, Member, Team, User};
use dioxus::prelude::*;

#[component]
pub fn Page(
    rbac: Rbac,
    members: Vec<Member>,
    invites: Vec<Invitation>,
    team: Team,
    user: User,
    can_manage_team: bool,
    team_name: String,
) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Team,
            team_id: team.id,
            rbac: rbac,
            title: "Team Members",
            header: rsx!(
                h3 { "Team Members" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-invite-form",
                    button_scheme: ButtonScheme::Primary,
                    "Invite New Team Member"
                }
            ),

            // If the user hasn't set their org name or their own name
            // get them to do it.
            if can_manage_team && (user.first_name.is_none() || team.name.is_none()) {
                Box {
                    class: "mb-3",
                    BoxHeader {
                        title: "Before you are able to invite people to your team you will need to do the following"
                    }
                    BoxBody {
                        if team.name.is_none() {
                            p {
                                "Please set your "
                                a {
                                    href: "#",
                                    "data-drawer-target": "set-name-drawer",
                                    "teams name"
                                }
                            }
                        }
                        if user.first_name.is_none() {
                            p {
                                "Please set your "
                                a {
                                    href: crate::routes::profile::Profile{team_id: team.id}.to_string(),
                                    "name"
                                }
                            }
                        }
                    }
                }
            }

            Box {
                class: "has-data-table",
                BoxHeader {
                    title: &team_name,
                    Button {
                        class: "ml-2",
                        drawer_trigger: "set-name-drawer",
                        button_size: ButtonSize::Small,
                        "Edit Name"
                    }
                }
                BoxBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Name or Email" }
                            th { "Status" }
                            th { "Special Privelages" }
                            if can_manage_team {
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                        }
                        tbody {
                            for member in &members {
                                tr {
                                    td {
                                        if let (Some(first_name), Some(last_name)) = (&member.first_name, &member.last_name) {
                                            Avatar {
                                                name: "{first_name}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{first_name} {last_name}"
                                            }
                                        } else {
                                            Avatar {
                                                name: "{member.email}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{member.email}"
                                            }
                                        }
                                    }
                                    td {
                                        Label {
                                            label_role: LabelRole::Success,
                                            "Active"
                                        }
                                    }
                                    td {
                                        for role in member.roles.clone() {
                                            super::team_role::Role {
                                                role: role
                                            }
                                        }
                                    }
                                    if can_manage_team {
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("remove-member-trigger-{}-{}",
                                                        member.id, member.team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "Remove User From Team"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            for invite in &invites {
                                tr {
                                    td {
                                            Avatar {
                                                name: "{invite.first_name}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{invite.first_name} {invite.last_name}"
                                            }
                                    }
                                    td {
                                        Label {
                                            label_role: LabelRole::Highlight,
                                            "Invite Pending"
                                        }
                                    }
                                    td {
                                        for role in invite.roles.clone() {
                                            super::team_role::Role {
                                                role
                                            }
                                        }
                                    }
                                    td {
                                        class: "text-right",
                                        DropDown {
                                            button_text: "...",
                                            direction: Direction::Left
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for member in members {
                super::remove_member::RemoveMemberDrawer {
                    team_id: member.team_id,
                    user_id: member.id,
                    email: member.email.clone(),
                    trigger_id: format!("remove-member-trigger-{}-{}", member.id, member.team_id)
                }
            }

            // The form to create an invitation
            super::invitation_form::InvitationForm {
                submit_action: crate::routes::team::CreateInvite{team_id:team.id}.to_string()
            }

            // Form to set he org name
            super::team_name_form::TeamNameForm {
                submit_action: crate::routes::team::SetName{team_id:team.id}.to_string()
            }
        }
    }
}
