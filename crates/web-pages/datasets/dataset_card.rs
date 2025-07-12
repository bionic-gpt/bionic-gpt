#![allow(non_snake_case)]
use crate::assistants::visibility::VisLabel;
use crate::components::card_item::{CardItem, CountLabel};
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::datasets::Dataset;
use dioxus::prelude::*;

#[component]
pub fn DatasetCard(team_id: i32, rbac: Rbac, dataset: Dataset) -> Element {
    rsx!(CardItem {
        class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
        clickable_link: crate::routes::documents::Index {
            team_id,
            dataset_id: dataset.id
        }
        .to_string(),
        title: dataset.name.clone(),
        description: None,
        footer: None,
        count_labels: vec![CountLabel {
            count: dataset.count as usize,
            label: "Document".into()
        }],
        action: Some(rsx!(
            div {
                class: "flex gap-2",
                VisLabel { visibility: dataset.visibility }
                DropDown {
                    direction: Direction::Left,
                    button_text: "...",
                    DropDownLink { href: crate::routes::documents::Index{team_id, dataset_id: dataset.id}.to_string(), target: "_top", "View" }
                    if rbac.can_edit_dataset(&dataset) {
                        DropDownLink { popover_target: format!("edit-trigger-{}-{}", dataset.id, team_id), href: "#", target: "_top", "Edit" }
                    }
                    DropDownLink { popover_target: format!("delete-trigger-{}-{}", dataset.id, team_id), href: "#", target: "_top", "Delete" }
                }
            }
        )),
    })
}
