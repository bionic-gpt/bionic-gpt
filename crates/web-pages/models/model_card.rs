#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use daisy_rsx::*;
use db::queries::models::ModelWithPrompt;
use dioxus::prelude::*;

#[component]
pub fn ModelCard(
    team_id: i32,
    model: ModelWithPrompt,
    has_function_calling: bool,
    has_vision: bool,
    has_tool_use: bool,
) -> Element {
    let display_name = if model.display_name.is_empty() {
        model.name.clone()
    } else {
        model.display_name.clone()
    };

    rsx!(CardItem {
        title: display_name.clone(),
        description: Some(rsx!(div {
            class: "flex gap-2 mt-1 text-xs",
            super::model_type::Model { model_type: model.model_type }
            if has_function_calling { Badge { badge_style: BadgeStyle::Ghost, "Functions" } }
            if has_vision { Badge { badge_style: BadgeStyle::Ghost, "Vision" } }
            if has_tool_use { Badge { badge_style: BadgeStyle::Ghost, "Tools" } }
        })),
        footer: None,
        image_src: None,
        avatar_name: None,
        count_labels: vec![
            CountLabel {
                count: model.tpm_limit as usize,
                label: "TPM".into()
            },
            CountLabel {
                count: model.rpm_limit as usize,
                label: "RPM".into()
            },
            CountLabel {
                count: model.context_size as usize,
                label: "Context".into()
            },
        ],
        action: Some(rsx!(
            DropDown {
                direction: Direction::Bottom,
                button_text: "...",
                DropDownLink { href: crate::routes::models::Edit{team_id, id: model.id}.to_string(), "Edit" }
                DropDownLink { popover_target: format!("delete-trigger-{}-{}", model.id, team_id), href: "#", target: "_top", "Delete" }
            }
        )),
        class: None,
        popover_target: None,
    })
}
