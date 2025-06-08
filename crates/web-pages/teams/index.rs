#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::ConfirmModal;
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{InviteSummary, TeamOwner};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    teams: Vec<TeamOwner>,
    invites: Vec<InviteSummary>,
    current_user_email: String,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Switch,
            team_id: team_id,
            rbac: rbac,
            title: "Your Teams",
            header: rsx!(
                h3 { "Your Teams" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "Create a New Team"
                }
            ),
            Card {
                class: "has-data-table",
                CardHeader {
                    title: "Teams"
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Team" }
                            th { "Team Creator" }
                            th {
                                class: "text-right",
                                "Action"
                            }
                        }
                        tbody {
                            for team in &teams {

                                if let Some(name) = &team.team_name {
                                    tr {
                                        td {
                                            Avatar {
                                                name: "{name}",
                                                avatar_type: avatar::AvatarType::Team
                                            }
                                            span {
                                                class: "ml-2 mr-2",
                                                "{name}"
                                            }
                                            if team.id != team_id {
                                                a {
                                                    "data-turbo-frame": "_top",
                                                    href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                    "(Switch to this Team)"
                                                }
                                            }
                                        }
                                        td {
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                        if team.team_owner == current_user_email && teams.len() > 1 {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        popover_target: format!("delete-trigger-{}", team.id),
                                                        href: "#",
                                                        target: "_top",
                                                        "Delete Team"
                                                    }
                                                }
                                            }
                                        } else {
                                            td {
                                                class: "text-right",
                                            }
                                        }
                                    }
                                } else {
                                    tr {
                                        td {
                                            Avatar {
                                                avatar_type: avatar::AvatarType::Team
                                            }
                                            span {
                                                class: "ml-2 mr-2",
                                                "Name Not Set"
                                            }
                                            if team.id != team_id {
                                                a {
                                                    "data-turbo-frame": "_top",
                                                    href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                    "(Switch to this Team)"
                                                }
                                            }
                                        }
                                        td {
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                        if team.team_owner == current_user_email && teams.len() > 1 {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        popover_target: format!("delete-trigger-{}", team.id),
                                                        href: "#",
                                                        target: "_top",
                                                        "Delete Team"
                                                    }
                                                }
                                            }
                                        } else {
                                            td {
                                                class: "text-right",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }


            Card {
                class: "has-data-table mt-8",
                CardHeader {
                    title: "You have invitations to join the following teams"
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Team" }
                            th {
                                "Team Creator"
                            }
                            th {
                                class: "text-right",
                                "Action"
                            }
                        }
                        tbody {
                            for invite in &invites {
                                td {
                                    "{invite.team_name}"
                                }
                                td {
                                    "{invite.created_by}"
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            popover_target: format!("accept-invite-trigger-{}", invite.id),
                                            href: "#",
                                            target: "_top",
                                            "Accept Invite"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for invite in invites {
                super::accept_invitation::AcceptInvite {
                    invite,
                    team_id
                }
            }

            for team in teams {
                ConfirmModal {
                    action: crate::routes::teams::Delete {team_id: team.id}.to_string(),
                    trigger_id: format!("delete-trigger-{}", team.id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Team?".to_string(),
                    warning: "Are you sure you want to delete this Team?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team.id.to_string()),
                    ],
                }
            }

            // The for to create new teams
            form {
                method: "post",
                "data-turbo-frame": "_top",
                action: crate::routes::teams::New{team_id}.to_string(),
                Modal {
                    trigger_id: "create-new-team",
                    ModalBody {
                        h3 {
                            class: "font-bold text-lg mb-4",
                            "Create a new team?"
                        }
                        div {
                            class: "flex flex-col",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Team Name",
                                help_text: "Give your new team a name",
                                required: true,
                                label: "Name",
                                name: "name"
                            }
                        }
                        ModalAction {
                            Button {
                                button_type: ButtonType::Submit,
                                button_scheme: ButtonScheme::Primary,
                                "Create Team"
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
