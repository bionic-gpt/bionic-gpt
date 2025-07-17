#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use daisy_rsx::*;
use db::queries::documents::Document;
use dioxus::prelude::*;

#[component]
pub fn DocumentCard(document: Document, team_id: i32, first_time: bool) -> Element {
    let text = if let Some(failure_reason) = document.failure_reason.clone() {
        failure_reason.replace(['{', '"', ':', '}'], " ")
    } else {
        "None".to_string()
    };

    let class = if document.waiting > 0 || document.batches == 0 {
        "processing"
    } else {
        "processing-finished"
    };

    let id = format!("processing-label-{}", document.id);

    let src = if first_time {
        Some(
            crate::routes::documents::Processing {
                team_id,
                document_id: document.id,
            }
            .to_string(),
        )
    } else {
        None
    };

    let status = rsx! {
        if document.waiting > 0 || document.batches == 0 {
            turbo-frame {
                id,
                src,
                Badge {
                    class: class,
                    badge_style: BadgeStyle::Outline,
                    badge_size: BadgeSize::Sm,
                    "Processing ({document.waiting} remaining)"
                }
            }
        } else if document.failure_reason.is_some() {
            turbo-frame {
                id,
                src,
                ToolTip {
                    text: "{text}",
                    Badge {
                        badge_color: BadgeColor::Error,
                        badge_style: BadgeStyle::Outline,
                        badge_size: BadgeSize::Sm,
                        "Failed"
                    }
                }
            }
        } else if document.batches == 0 {
            turbo-frame { id, src, Badge { badge_style: BadgeStyle::Outline, badge_size: BadgeSize::Sm, "Queued" } }
        } else if document.fail_count > 0 {
            turbo-frame { id, src, Badge { badge_color: BadgeColor::Error, badge_style: BadgeStyle::Outline, badge_size: BadgeSize::Sm, "Processed ({document.fail_count} failed)" } }
        } else if document.failure_reason.is_some() {
            turbo-frame { id, src, Badge { badge_color: BadgeColor::Error, badge_style: BadgeStyle::Outline, badge_size: BadgeSize::Sm, "Failed" } }
        } else {
            turbo-frame { id, src, Badge { badge_color: BadgeColor::Success, badge_style: BadgeStyle::Outline, badge_size: BadgeSize::Sm, "Processed" } }
        }
    };

    rsx!(CardItem {
        title: document.file_name.clone(),
        description: Some(rsx!( span { "{document.content_size} bytes" } )),
        footer: Some(status),
        count_labels: vec![CountLabel {
            count: document.batches as usize,
            label: "Chunk".into()
        }],
        action: Some(rsx!(
            DropDown {
                direction: Direction::Left,
                button_text: "...",
                DropDownLink {
                    popover_target: format!("delete-doc-trigger-{}-{}", document.id, team_id),
                    href: "#",
                    target: "_top",
                    "Delete Document"
                }
            }
        )),
        class: None,
        popover_target: None,
        image_src: None,
        avatar_name: None,
    })
}
