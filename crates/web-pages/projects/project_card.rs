#![allow(non_snake_case)]
use crate::components::card_item::{CardItem, CountLabel};
use crate::visibility_to_string;
use daisy_rsx::*;
use db::queries::projects::ProjectSummary;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectCardProps {
    pub project: ProjectSummary,
    pub team_id: i32,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    let project = props.project;
    let name = project.name.clone();
    let project_link = crate::routes::projects::View {
        team_id: props.team_id,
        project_id: project.id,
    }
    .to_string();

    let conversation_count = project.conversation_count as usize;
    let attachment_count = project.attachment_count as usize;

    rsx!(CardItem {
        class: Some("cursor-pointer hover:bg-base-200 w-full".into()),
        clickable_link: Some(project_link.clone()),
        avatar_name: Some(name.clone()),
        title: name,
        description: Some(rsx!(span { "Visibility: {visibility_to_string(project.visibility)}" })),
        footer: None,
        count_labels: vec![
            CountLabel {
                count: conversation_count,
                label: "Chat".to_string(),
            },
            CountLabel {
                count: attachment_count,
                label: "Attachment".to_string(),
            },
        ],
        action: Some(rsx!(
            div {
                class: "flex flex-col items-end gap-2",
                Button {
                    button_type: ButtonType::Link,
                    button_scheme: ButtonScheme::Neutral,
                    button_size: ButtonSize::Small,
                    target: "_top",
                    href: project_link.clone(),
                    "Open"
                }
                Button {
                    button_scheme: ButtonScheme::Neutral,
                    button_size: ButtonSize::Small,
                    popover_target: format!("edit-project-{}-{}", project.id, props.team_id),
                    "Edit"
                }
                Button {
                    button_scheme: ButtonScheme::Neutral,
                    button_size: ButtonSize::Small,
                    popover_target: format!("delete-project-{}-{}", project.id, props.team_id),
                    "Delete"
                }
            }
        )),
    })
}
