#![allow(non_snake_case)]
use daisy_rsx::*;
use db::TeamOwner;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TeamCardProps {
    pub team: TeamOwner,
    pub current_team_id: i32,
    pub teams_len: usize,
    pub current_user_email: String,
}

#[component]
pub fn TeamCard(props: TeamCardProps) -> Element {
    let name = props
        .team
        .team_name
        .clone()
        .unwrap_or_else(|| "Name Not Set".to_string());
    rsx!(
        Card {
            class: "p-3 flex flex-row justify-between",
            div {
                class: "flex flex-row items-center",
                Avatar {
                    avatar_size: AvatarSize::Medium,
                    avatar_type: AvatarType::Team,
                    name: "{name}"
                }
                div {
                    class: "ml-4 flex flex-col",
                    h2 {
                        class: "font-semibold text-base mb-1",
                        "{name}"
                    }
                    p {
                        class: "text-sm text-base-content/70",
                        "{props.team.team_owner}"
                    }
                    if props.team.id != props.current_team_id {
                        a {
                            href: crate::routes::team::Index{ team_id: props.team.id }.to_string(),
                            class: "text-xs text-primary",
                            "Switch to this Team"
                        }
                    }
                }
            }
            if props.team.team_owner == props.current_user_email && props.teams_len > 1 {
                div {
                    class: "flex flex-col justify-center ml-4",
                    DropDown {
                        direction: Direction::Left,
                        button_text: "...",
                        DropDownLink {
                            popover_target: format!("delete-trigger-{}", props.team.id),
                            href: "#",
                            target: "_top",
                            "Delete Team"
                        }
                    }
                }
            }
        }
    )
}
