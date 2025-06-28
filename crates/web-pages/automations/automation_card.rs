#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use crate::routes::prompts::Image;
use daisy_rsx::*;
use db::queries::prompts::MyPrompt;
use dioxus::prelude::*;

#[component]
pub fn AutomationCard(team_id: i32, prompt: MyPrompt) -> Element {
    let description: String = prompt
        .description
        .chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r')
        .collect();

    rsx! {
        CardItem {
            image_src: prompt.image_icon_object_id.map(|id| Image { team_id, id }.to_string()),
            avatar_name: Some(prompt.name.clone()),
            title: prompt.name.clone(),
            description: if description.is_empty() { None } else { Some(rsx!( span { "{description}" } )) },
            footer: Some(rsx!( span { "Last updated " RelativeTime { format: RelativeTimeFormat::Relative, datetime: "{prompt.updated_at}" } } )),
            count_labels: vec![
                CountLabel { count: prompt.integration_count as usize, label: "Integration".into() },
                CountLabel { count: prompt.trigger_count as usize, label: "Trigger".into() },
            ],
            action: Some(rsx!(
                DropDown {
                    direction: Direction::Bottom,
                    button_text: "...",
                    DropDownLink { href: crate::routes::automations::Edit{team_id, prompt_id: prompt.id}.to_string(), "Edit" }
                    DropDownLink { href: crate::routes::automations::ManageIntegrations{team_id, prompt_id: prompt.id}.to_string(), "Manage Integrations" }
                    DropDownLink { href: crate::routes::automations::ManageTriggers{team_id, prompt_id: prompt.id}.to_string(), "Manage Triggers" }
                    DropDownLink { popover_target: format!("delete-trigger-{}-{}", prompt.id, team_id), href: "#", target: "_top", "Delete" }
                }
            ))
        }
    }
}
