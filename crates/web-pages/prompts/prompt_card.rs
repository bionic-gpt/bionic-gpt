#![allow(non_snake_case)]
use crate::routes::prompts::Image;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn PromptCard(team_id: i32, rbac: Rbac, prompt: Prompt) -> Element {
    rsx! {
        Box {
            class: "cursor-pointer hover:bg-base-200",
            drawer_trigger: format!("view-trigger-{}-{}", prompt.id, team_id),
            BoxHeader {
                class: "truncate ellipses flex justify-between",
                title: "{prompt.name}",
                super::visibility::VisLabel {
                    visibility: prompt.visibility
                }
            }
            BoxBody {
                div {
                    class: "flex w-full",
                    if let Some(object_id) = prompt.image_icon_object_id {
                        crate::avatar::Avatar {
                            avatar_size: crate::avatar::AvatarSize::ExtraLarge,
                            image_src: Image { team_id, id: object_id }.to_string()
                        }
                    } else {
                        crate::avatar::Avatar {
                            avatar_size: crate::avatar::AvatarSize::ExtraLarge,
                            avatar_type: crate::avatar::AvatarType::User
                        }
                    }
                    div {
                        p {
                            class: "ml-8 text-sm",
                            "{prompt.description}"
                        }
                        div {
                            class: "ml-8 mt-3 text-xs flex justify-center gap-1",
                            "Last update",
                            RelativeTime {
                                format: RelativeTimeFormat::Relative,
                                datetime: "{prompt.updated_at}"
                            }
                        }
                    }
                }
            }
        }
    }
}
