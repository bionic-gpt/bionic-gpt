#![allow(non_snake_case)]
use super::{invite_card::InviteCard, team_card::TeamCard};
use crate::app_layout::{Layout, SideBar};
use crate::components::confirm_modal::ConfirmModal;
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
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: "Your Teams".into(),
                        href: None
                    }]
                }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    popover_target: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "Create a New Team"
                }
            ),
            div {
                class: "p-4 max-w-3xl w-full mx-auto",

                h1 {
                    class: "text-xl font-semibold",
                    "Teams"
                }
                p {
                    "You are a member of the following teams"
                }

                div {
                    class: "mt-5 space-y-2",
                    for team in &teams {
                        TeamCard {
                            team: team.clone(),
                            current_team_id: team_id,
                            teams_len: teams.len(),
                            current_user_email: current_user_email.clone(),
                        }
                    }
                }

                if ! invites.is_empty() {
                    h1 {
                        class: "mt-5 text-xl font-semibold",
                        "Invitations"
                    }
                    p {
                        "You have invitations to join the following teams"
                    }

                    div {
                        class: "space-y-2",
                        for invite in &invites {
                            InviteCard { invite: invite.clone() }
                        }
                    }

                    for invite in invites {
                        super::accept_invitation::AcceptInvite {
                            invite,
                            team_id
                        }
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
                                Fieldset {
                                    legend: "Name",
                                    help_text: "Give your new team a name",
                                    Input {
                                        input_type: InputType::Text,
                                        placeholder: "Team Name",
                                        required: true,
                                        name: "name"
                                    }
                                }
                            }
                            ModalAction {
                                Button {
                                    class: "cancel-modal",
                                    button_scheme: ButtonScheme::Warning,
                                    button_size: ButtonSize::Small,
                                    "Cancel"
                                }
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
    };

    crate::render(page)
}
