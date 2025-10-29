#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use daisy_rsx::*;
use db::TeamOwner;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TeamCardProps {
    pub team: TeamOwner,
    pub current_team_id: i32,
    pub teams_len: usize,
    pub current_user_email: String,
    pub member_count: usize,
}

#[component]
pub fn TeamCard(props: TeamCardProps) -> Element {
    let name = props
        .team
        .team_name
        .clone()
        .unwrap_or_else(|| "Name Not Set".to_string());
    let owner_email = props.team.team_owner.clone();
    let team_link = crate::routes::team::Index {
        team_id: props.team.id,
    }
    .to_string();

    rsx!(CardItem {
        class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
        clickable_link: Some(team_link),
        avatar_name: Some(name.clone()),
        title: name,
        description: Some(rsx!(span { "Owner: {owner_email}" })),
        footer: None,
        count_labels: vec![CountLabel {
            count: props.member_count,
            label: "Member".into()
        }],
        action: Some(rsx!(
            div {
                class: "flex flex-col items-end gap-2",
                if props.team.id != props.current_team_id {
                    Button {
                        button_type: ButtonType::Link,
                        target: "_top",
                        href: crate::routes::teams::Switch { team_id: props.team.id }.to_string(),
                        button_scheme: ButtonScheme::Info,
                        button_size: ButtonSize::Small,
                        "Switch to this Team"
                    }
                }
                if props.team.team_owner == props.current_user_email && props.teams_len > 1 {
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
        )),
    })
}
