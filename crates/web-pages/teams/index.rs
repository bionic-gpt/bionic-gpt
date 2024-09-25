#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{InviteSummary, TeamOwner};
use dioxus::prelude::*;

#[component]
pub fn Page(
    rbac: Rbac,
    team_id: i32,
    teams: Vec<TeamOwner>,
    invites: Vec<InviteSummary>,
    current_user_email: String,
) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Switch,
            team_id: team_id,
            rbac: rbac,
            title: "Your Teams",
            header: rsx!(
                h3 { "Your Teams" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "Create a New Team"
                }
            ),
            Box {
                class: "has-data-table",
                BoxHeader {
                    title: "Teams"
                }
                BoxBody {
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
                                        if team.team_owner == current_user_email {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        drawer_trigger: format!("delete-trigger-{}", team.id),
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
                                        if team.team_owner == current_user_email {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        drawer_trigger: format!("delete-trigger-{}", team.id),
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


            Box {
                class: "has-data-table mt-8",
                BoxHeader {
                    title: "You have invitations to join the following teams"
                }
                BoxBody {
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
                                            drawer_trigger: format!("accept-invite-trigger-{}", invite.id),
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
                super::delete::DeleteDrawer {
                    team_id: team.id,
                    trigger_id: format!("delete-trigger-{}", team.id)
                }
            }

            // The for to create new teams
            form {
                method: "post",
                "data-turbo-frame": "_top",
                action: crate::routes::teams::New{team_id}.to_string(),
                Drawer {
                    label: "Create a new team?",
                    trigger_id: "create-new-team",
                    DrawerBody {
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
                    }
                    DrawerFooter {
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
}
