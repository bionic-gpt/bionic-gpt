#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use crate::team::team_name_form::TeamNameForm;
use daisy_rsx::*;
use db::TeamOwner;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TeamCardProps {
    pub team: TeamOwner,
    pub member_count: usize,
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
    let edit_trigger = format!("edit-team-name-{}", props.team.id);
    rsx!(
        CardItem {
            class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
            clickable_link: Some(crate::routes::team::Index { team_id: props.team.id }.to_string()),
            popover_target: None,
            image_src: None,
            avatar_name: Some(name.clone()),
            title: name.clone(),
            description: Some(rsx!(span { "{props.team.team_owner}" })),
            footer: None,
            count_labels: vec![CountLabel { count: props.member_count, label: "Member".into() }],
            action: Some(rsx!(
                if props.team.team_owner == props.current_user_email && props.teams_len > 1 {
                    DropDown {
                        direction: Direction::Left,
                        button_text: "...",
                        DropDownLink { popover_target: edit_trigger.clone(), href: "#", target: "_top", "Edit Name" }
                        DropDownLink { popover_target: format!("delete-trigger-{}", props.team.id), href: "#", target: "_top", "Delete Team" }
                    }
                }
                if props.team.id != props.current_team_id {
                    Button {
                        button_type: ButtonType::Link,
                        target: "_top",
                        href: crate::routes::teams::Switch { team_id: props.team.id }.to_string(),
                        button_scheme: ButtonScheme::Info,
                        button_size: ButtonSize::Small,
                        "Switch"
                    }
                }
            )),
        }

        TeamNameForm {
            submit_action: crate::routes::team::SetName { team_id: props.team.id }.to_string(),
            trigger_id: edit_trigger,
        }
    )
}
