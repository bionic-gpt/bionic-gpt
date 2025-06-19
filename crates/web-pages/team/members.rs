#![allow(non_snake_case)]
use super::member_card::{InvitePendingCard, MemberCard};
use crate::app_layout::{Layout, SideBar};
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Invitation, Member, Team, User};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    members: Vec<Member>,
    invites: Vec<Invitation>,
    team: Team,
    user: User,
    team_name: String,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Team,
            team_id: team.id,
            rbac: rbac.clone(),
            title: "Team Members",
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Team Members".into(),
                        href: None
                    }]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "create-invite-form",
                    button_scheme: ButtonScheme::Primary,
                    "Invite New Team Member"
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                // If the user hasn't set their org name or their own name
                // get them to do it.
                if rbac.can_make_invitations() && (user.first_name.is_none() || team.name.is_none()) {
                    Card {
                        class: "mb-3",
                        CardHeader {
                            title: "Before you are able to invite people to your team you will need to do the following"
                        }
                        CardBody {
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


                h1 {
                    class: "mt-5 text-xl font-semibold",
                    "{team_name}",
                    Button {
                        class: "ml-2",
                        popover_target: "set-name-drawer",
                        button_size: ButtonSize::Small,
                        "Edit Name"
                    }
                }
                p {
                    "These people are a member of this team"
                }

                div {
                    class: "mt-5 space-y-2",
                    for member in &members {
                        MemberCard { member: member.clone(), rbac: rbac.clone() }
                    }
                    for invite in &invites {
                        InvitePendingCard { invite: invite.clone(), rbac: rbac.clone() }
                    }
                }

                for member in members {
                    ConfirmModal {
                        action: crate::routes::team::Delete{team_id: member.team_id}.to_string(),
                        trigger_id: format!("remove-member-trigger-{}-{}", member.id, member.team_id),
                        submit_label: "Remove User".to_string(),
                        heading: "Remove this user?".to_string(),
                        warning: format!("Are you sure you want to remove '{}' from the team?", member.email),
                        hidden_fields: vec![
                            ("team_id".into(), member.team_id.to_string()),
                            ("user_id".into(), member.id.to_string()),
                        ],
                    }
                }

                for invite in invites {
                    ConfirmModal {
                        action: crate::routes::team::DeleteInvite{team_id: invite.team_id}.to_string(),
                        trigger_id: format!("remove-invite-trigger-{}-{}", invite.id, invite.team_id),
                        submit_label: "Remove Invite".to_string(),
                        heading: "Remove this invite?".to_string(),
                        warning: "Are you sure you want to remove this invite?".to_string(),
                        hidden_fields: vec![
                            ("team_id".into(), invite.team_id.to_string()),
                            ("invite_id".into(), invite.id.to_string()),
                        ],
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
    };

    crate::render(page)
}
