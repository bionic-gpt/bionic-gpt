#![allow(non_snake_case)]
use crate::{console::empty_stream::ExampleForm, routes::prompts::Image};
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ViewDrawer(team_id: i32, prompt: Prompt, trigger_id: String) -> Element {
    rsx! {
        Drawer {
            label: "{prompt.name}",
            trigger_id,
            DrawerBody {
                div {
                    class: "text-center",
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
                }
                h2 {
                    class: "text-center text-xl font-semibold",
                    "{prompt.name}"
                }
                p {
                    class: "mt-6 text-center text-sm text-token-text-tertiary",
                    "Created by {prompt.author_name}"
                }
                p {
                    class: "mt-6 text-center",
                    "{prompt.description}"
                }
                h2 {
                    class: "mt-12 mb-8 text-xl font-semibold",
                    "Conversation Starters"
                }
                div {
                    class: "flex flex-col gap-4",
                    if let Some(example) = prompt.example1 {
                        if ! example.is_empty() {
                            ExampleForm {
                                conversation_id: 1,
                                team_id,
                                prompt_id: prompt.id,
                                example: example
                            }
                        }
                    }
                    if let Some(example) = prompt.example2 {
                        if ! example.is_empty() {
                            ExampleForm {
                                conversation_id: 1,
                                team_id,
                                prompt_id: prompt.id,
                                example: example
                            }
                        }
                    }
                    if let Some(example) = prompt.example3 {
                        if ! example.is_empty() {
                            ExampleForm {
                                conversation_id: 1,
                                team_id,
                                prompt_id: prompt.id,
                                example: example
                            }
                        }
                    }
                    if let Some(example) = prompt.example4 {
                        if ! example.is_empty() {
                            ExampleForm {
                                conversation_id: 1,
                                team_id,
                                prompt_id: prompt.id,
                                example: example
                            }
                        }
                    }
                }
            }
            DrawerFooter {
                a {
                    class: "btn btn-primary btn-sm w-full",
                    href: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                    "Start a Chat"
                }
            }
        }
    }
}
