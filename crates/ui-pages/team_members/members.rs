#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Invitation, Member, Team, User};
use dioxus::prelude::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    rbac: Rbac,
    members: Vec<Member>,
    invites: Vec<Invitation>,
    team: Team,
    user: User,
    can_manage_team: bool,
    team_name: String,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Team,
            team_id: team.id,
            rbac: rbac,
            title: "Team Members",
            header: cx.render(rsx!(
                h3 { "Team Members" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-invite-form",
                    button_scheme: ButtonScheme::Primary,
                    "Invite New Team Member"
                }
            )),

            // If the user hasn't set their org name or their own name
            // get them to do it.
            if *can_manage_team && (user.first_name.is_none() || team.name.is_none()) {

                cx.render(rsx! {
                    Box {
                        class: "mb-3",
                        BoxHeader {
                            title: "Before you are able to invite people to your team you will need to do the following"
                        }
                        BoxBody {
                            if team.name.is_none() {
                                cx.render(rsx! {
                                    p {
                                        "Please set your "
                                        a {
                                            href: "#",
                                            "data-drawer-target": "set-name-drawer",
                                            "teams name"
                                        }
                                    }
                                })
                            } else {
                                None
                            }
                            if user.first_name.is_none() {
                                cx.render(rsx! {
                                    p {
                                        "Please set your "
                                        a {
                                            href: "{crate::routes::profile::index_route(team.id)}",
                                            "name"
                                        }
                                    }
                                })
                            } else {
                                None
                            }
                        }
                    }
                })
            } else {
                None
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
                            if *can_manage_team {
                                cx.render(rsx!(
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                ))
                            } else {
                                None
                            }
                        }
                        tbody {
                            members.iter().map(|member| rsx!(
                                tr {
                                    td {
                                        if let (Some(first_name), Some(last_name)) = (&member.first_name, &member.last_name) {
                                            cx.render(rsx!(
                                                Avatar {
                                                    name: "{first_name}",
                                                    avatar_type: avatar::AvatarType::User
                                                }
                                                span {
                                                    class: "ml-2",
                                                    "{first_name} {last_name}"
                                                }
                                            ))
                                        } else {
                                            cx.render(rsx!(
                                                Avatar {
                                                    name: "{member.email}",
                                                    avatar_type: avatar::AvatarType::User
                                                }
                                                span {
                                                    class: "ml-2",
                                                    "{member.email}"
                                                }
                                            ))
                                        }
                                    }
                                    td {
                                        Label {
                                            label_role: LabelRole::Success,
                                            "Active"
                                        }
                                    }
                                    td {
                                        member.roles.iter().map(|role|
                                            cx.render(rsx!(
                                                super::team_role::Role {
                                                    role: *role
                                                }
                                            ))
                                        )
                                    }
                                    if *can_manage_team {
                                        cx.render(rsx!(
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
                                        ))
                                    } else {
                                        None
                                    }
                                }
                            ))
                            invites.iter().map(|invite| rsx!(
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
                                        invite.roles.iter().map(|role|
                                            cx.render(rsx!(
                                                super::team_role::Role {
                                                    role: *role
                                                }
                                            ))
                                        )
                                    }
                                    if *can_manage_team {
                                        cx.render(rsx!(
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "",
                                                    DropDownLink {
                                                        href: "#",
                                                        target: "_top",
                                                        "Resend Invite"
                                                    }
                                                }
                                            }
                                        ))
                                    } else {
                                        None
                                    }
                                }
                            ))
                        }
                    }
                }
            }

            members.iter().map(|member| rsx!(
                cx.render(rsx!(
                    super::remove_member::RemoveMemberDrawer {
                        team_id: member.team_id,
                        user_id: member.id,
                        email: member.email.clone(),
                        trigger_id: format!("remove-member-trigger-{}-{}", member.id, member.team_id)
                        //team_id: &team.id
                    }
                ))
            ))

            // The form to create an invitation
            super::invitation_form::InvitationForm {
                submit_action: crate::routes::team::create_route(team.id)
            }

            // Form to set he org name
            super::team_name_form::TeamNameForm {
                submit_action: crate::routes::team::set_name_route(team.id)
            }
        }
    })
}

pub fn members(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
