#![allow(non_snake_case)]
use crate::{console::empty_stream::ExampleForm, routes::prompts::Image};
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ViewDrawer(team_id: i32, prompt: Prompt, trigger_id: String) -> Element {
    let example1 = prompt.example1.unwrap_or("".to_string());
    let example2 = prompt.example2.unwrap_or("".to_string());
    let example3 = prompt.example3.unwrap_or("".to_string());
    let example4 = prompt.example4.unwrap_or("".to_string());
    rsx! {
        Drawer {
            label: "{prompt.name}",
            trigger_id,
            DrawerBody {
                div {
                    class: "text-center",
                    if let Some(object_id) = prompt.image_icon_object_id {
                        Avatar {
                            avatar_size: AvatarSize::ExtraLarge,
                            image_src: Image { team_id, id: object_id }.to_string()
                        }
                    } else {
                        Avatar {
                            avatar_size: AvatarSize::ExtraLarge,
                            avatar_type: AvatarType::User
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
                if ! example2.is_empty() || ! example2.is_empty()  || ! example3.is_empty() || ! example4.is_empty() {
                    h2 {
                        class: "mt-12 mb-8 text-xl font-semibold",
                        "Conversation Starters"
                    }
                    div {
                        class: "flex flex-col gap-4",
                        if ! example1.is_empty() {
                            ExampleForm {
                                team_id,
                                prompt_id: prompt.id,
                                example: example1
                            }
                        }
                        if ! example2.is_empty() {
                            ExampleForm {
                                team_id,
                                prompt_id: prompt.id,
                                example: example2
                            }
                        }
                        if ! example3.is_empty() {
                            ExampleForm {
                                team_id,
                                prompt_id: prompt.id,
                                example: example3
                            }
                        }
                        if ! example4.is_empty() {
                            ExampleForm {
                                team_id,
                                prompt_id: prompt.id,
                                example: example4
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
