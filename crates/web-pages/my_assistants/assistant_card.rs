#![allow(non_snake_case)]
use crate::routes::prompts::Image;
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn MyAssistantCard(team_id: i32, prompt: Prompt) -> Element {
    let description: String = prompt
        .description
        .chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r')
        .collect();

    rsx! {
        Card {
            class: "p-3 mt-5 flex flex-row",
            a {
                href: crate::routes::prompts::View{team_id, prompt_id: prompt.id}.to_string(),
                class: "no-underline flex-1 min-w-0",
                div {
                    class: "flex flex-row",
                    // Left section: Image/Avatar
                    div {
                        class: "flex-shrink-0",
                        if let Some(object_id) = prompt.image_icon_object_id {
                            img {
                                class: "border border-neutral-content rounded p-2",
                                src: Image { team_id, id: object_id }.to_string(),
                                width: "48",
                                height: "48"
                            }
                        } else {
                            Avatar {
                                avatar_size: AvatarSize::Medium,
                                avatar_type: AvatarType::User
                            }
                        }
                        div {
                            class: "mt-2",
                            crate::assistants::visibility::VisLabel {
                                visibility: prompt.visibility
                            }
                        }
                    }
                    // Middle section: Info
                    div {
                        class: "ml-4 text-sm flex flex-col justify-center flex-1 min-w-0",
                        h2 {
                            class: "font-semibold text-base mb-1",
                            "{prompt.name}"
                        }
                        if !description.is_empty() {
                            p {
                                class: "text-sm text-gray-600 truncate overflow-hidden whitespace-nowrap mb-2",
                                "{description}"
                            }
                        }
                        div {
                            class: "flex items-center gap-2 text-xs text-gray-500",
                            span {
                                "Last updated "
                            }
                            RelativeTime {
                                format: RelativeTimeFormat::Relative,
                                datetime: "{prompt.updated_at}"
                            }
                        }
                    }
                }
            }
            // Right section: Action buttons
            div {
                class: "flex flex-col justify-center ml-4 gap-2",
                Button {
                    button_type: ButtonType::Link,
                    button_scheme: ButtonScheme::Neutral,
                    button_size: ButtonSize::Small,
                    href: crate::routes::prompts::Edit{team_id, prompt_id: prompt.id}.to_string(),
                    "Edit"
                }
                Button {
                    button_scheme: ButtonScheme::Error,
                    button_size: ButtonSize::Small,
                    popover_target: format!("delete-trigger-{}-{}", prompt.id, team_id),
                    "Delete"
                }
            }
        }
    }
}
