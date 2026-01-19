#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use daisy_rsx::*;
use db::Provider;
use dioxus::prelude::*;

#[component]
pub fn ProviderCard(team_id: i32, provider: Provider) -> Element {
    let default_display = provider
        .default_model_display_name
        .clone()
        .unwrap_or_else(|| provider.default_model_name.clone().unwrap_or_default());

    let default_label = if default_display.is_empty() {
        "No default model".to_string()
    } else {
        format!("Default model: {}", default_display)
    };

    rsx!(CardItem {
        title: provider.name.clone(),
        description: Some(rsx!(div {
            class: "flex flex-col gap-1 text-xs",
            span { class: "truncate", "{provider.base_url}" }
            span { class: "truncate", "{default_label}" }
        })),
        footer: None,
        image_html: Some(provider.svg_logo.clone()),
        image_src: None,
        avatar_name: None,
        count_labels: vec![CountLabel {
            count: provider.default_model_context_size as usize,
            label: "Context".into()
        }],
        action: Some(rsx!(
            DropDown {
                direction: Direction::Bottom,
                button_text: "...",
                DropDownLink { href: crate::routes::providers::Edit{team_id, id: provider.id}.to_string(), "Edit" }
                DropDownLink { popover_target: format!("delete-provider-{}-{}", provider.id, team_id), href: "#", target: "_top", "Delete" }
            }
        )),
        class: None,
        popover_target: None,
    })
}
