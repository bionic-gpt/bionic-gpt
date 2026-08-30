#![allow(non_snake_case)]

use crate::visibility_to_string;
use daisy_rsx::*;
use db::queries::projects::ProjectSummary;
use dioxus::prelude::*;

#[component]
pub fn ProjectCard(project: ProjectSummary, team_id: String) -> Element {
    let project_link = crate::routes::projects::View {
        team_id: team_id.clone(),
        project_id: project.id,
    }
    .to_string();
    let chat_label = if project.conversation_count == 1 {
        "1 chat".to_string()
    } else {
        format!("{} chats", project.conversation_count)
    };
    let file_label = if project.attachment_count == 1 {
        "1 file".to_string()
    } else {
        format!("{} files", project.attachment_count)
    };

    rsx! {
        div {
            class: "group flex items-center gap-3 rounded-xl px-3 py-3 hover:bg-base-200",
            a {
                class: "min-w-0 flex-1",
                href: project_link,
                h2 {
                    class: "truncate font-medium",
                    "{project.name}"
                }
                div {
                    class: "mt-1 flex flex-wrap items-center gap-x-2 text-xs text-base-content/55",
                    span { "{chat_label}" }
                    span { "·" }
                    span { "{file_label}" }
                    span { "·" }
                    span { "{visibility_to_string(project.visibility)}" }
                }
            }
            DropDown {
                direction: Direction::Left,
                button_text: "...",
                DropDownLink {
                    popover_target: format!("edit-project-{}-{}", project.id, team_id),
                    href: "#",
                    target: "_top",
                    "Edit project"
                }
                DropDownLink {
                    popover_target: format!("delete-project-{}-{}", project.id, team_id),
                    href: "#",
                    target: "_top",
                    "Delete project"
                }
            }
        }
    }
}
