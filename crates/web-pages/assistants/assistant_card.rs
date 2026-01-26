#![allow(non_snake_case)]
use crate::components::card_item::CardItem;
use crate::routes::prompts::Image;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn AssistantCard(team_id: String, rbac: Rbac, prompt: Prompt) -> Element {
    let description: String = prompt
        .description
        .chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r')
        .collect();
    rsx! {
        CardItem {
            class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
            popover_target: Some(format!("view-trigger-{}-{}", prompt.id, team_id)),
            image_src: prompt.image_icon_object_id.map(|id| Image { team_id: team_id.clone(), id }.to_string()),
            avatar_name: Some(prompt.name.clone()),
            title: prompt.name.clone(),
            description: Some(rsx!(span { "{description}" } )),
            footer: Some(rsx!( span { "Last update " RelativeTime { format: RelativeTimeFormat::Relative, datetime: "{prompt.updated_at}" } } )),
            count_labels: vec![],
            action: Some(rsx!(super::visibility::VisLabel { visibility: prompt.visibility })),
        }
    }
}
