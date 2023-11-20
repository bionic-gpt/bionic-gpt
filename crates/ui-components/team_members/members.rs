#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use db::{Invitation, Member, Organisation, User};
use dioxus::prelude::*;
use primer_rsx::*;

struct MembersProps {
    members: Vec<Member>,
    invites: Vec<Invitation>,
    organisation: Organisation,
    user: User,
    can_manage_team: bool,
    submit_action: String,
    team_name: String,
    profile_link: String,
    name_form_submit_action: String,
}

pub fn members(
    invites: Vec<Invitation>,
    members: Vec<Member>,
    organisation: Organisation,
    user: User,
    can_manage_team: bool,
) -> String {
    fn app(cx: Scope<MembersProps>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::Team,
                team_id: cx.props.organisation.id,
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
                if cx.props.can_manage_team && (cx.props.user.first_name.is_none() || cx.props.organisation.name.is_none()) {

                    cx.render(rsx! {
                        Box {
                            class: "mb-3",
                            BoxHeader {
                                title: "Before you are able to invite people to your team you will need to do the following"
                            }
                            BoxBody {
                                if cx.props.organisation.name.is_none() {
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
                                if cx.props.user.first_name.is_none() {
                                    cx.render(rsx! {
                                        p {
                                            "Please set your "
                                            a {
                                                href: "{cx.props.profile_link}",
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
                        title: &cx.props.team_name,
                        Button {
                            class: "ml-2",
                            drawer_trigger: "set-name-drawer",
                            button_size: ButtonSize::Small,
                            "Edit Name"
                        }
                    }
                    BoxBody {
                        DataTable {
                            table {
                                thead {
                                    th { "Name or Email" }
                                    th { "Status" }
                                    th { "Special Privelages" }
                                    if cx.props.can_manage_team {
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
                                    cx.props.members.iter().map(|member| rsx!(
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
                                                    label_color: LabelColor::Primary,
                                                    label_contrast: LabelContrast::Primary,
                                                    "Active"
                                                }
                                            }
                                            td {
                                                member.roles.iter().map(|role|
                                                    cx.render(rsx!(
                                                        super::team_role::Role {
                                                            role: role
                                                        }
                                                    ))
                                                )
                                            }
                                            if cx.props.can_manage_team {
                                                cx.render(rsx!(
                                                    td {
                                                        class: "text-right",
                                                        DropDown {
                                                            direction: Direction::Left,
                                                            button_text: "...",
                                                            DropDownLink {
                                                                drawer_trigger: format!("remove-member-trigger-{}-{}", 
                                                                    member.id, member.organisation_id),
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
                                    cx.props.invites.iter().map(|invite| rsx!(
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
                                                    label_color: LabelColor::Accent,
                                                    label_contrast: LabelContrast::Primary,
                                                    "Invite Pending"
                                                }
                                            }
                                            td {
                                                invite.roles.iter().map(|role|
                                                    cx.render(rsx!(
                                                        super::team_role::Role {
                                                            role: role
                                                        }
                                                    ))
                                                )
                                            }
                                            if cx.props.can_manage_team {
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
                }
            }

            cx.props.members.iter().map(|member| rsx!(
                cx.render(rsx!(
                    super::remove_member::RemoveMemberDrawer {
                        organisation_id: member.organisation_id,
                        user_id: member.id,
                        email: member.email.clone(),
                        trigger_id: format!("remove-member-trigger-{}-{}", member.id, member.organisation_id)
                        //organisation_id: &organisation.id
                    }
                ))
            ))

            // The form to create an invitation
            super::invitation_form::InvitationForm {
                submit_action: cx.props.submit_action.clone()
            }

            // Form to set he org name
            super::team_name_form::TeamNameForm {
                submit_action: cx.props.name_form_submit_action.clone()
            }
        })
    }

    let submit_action = crate::routes::team::create_route(organisation.id);
    let profile_link = crate::routes::profile::index_route(organisation.id);
    let name_form_submit_action = crate::routes::team::set_name_route(organisation.id);

    let team_name = if let Some(team) = &organisation.name {
        format!("Team : {}", team)
    } else {
        "Team : No Name ".to_string()
    };

    crate::render(VirtualDom::new_with_props(
        app,
        MembersProps {
            members,
            invites,
            organisation,
            user,
            can_manage_team,
            submit_action,
            team_name,
            profile_link,
            name_form_submit_action,
        },
    ))
}
