#![allow(non_snake_case)]
use crate::{console::empty_stream::ExampleForm, routes::prompts::Image};
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ViewDrawer(team_id: i32, prompt: Prompt, trigger_id: String) -> Element {
    let examples: Vec<Option<String>> = vec![
        prompt.example1,
        prompt.example2,
        prompt.example3,
        prompt.example4,
    ];
    let has_some_examples = examples.iter().any(|e| e.is_some());
    rsx! {
        Modal {
            trigger_id,
            ModalBody {
                div {
                    class: "flex justify-center",
                    if let Some(object_id) = prompt.image_icon_object_id {
                        img {
                            width: "96",
                            height: "96",
                            src: Image { team_id, id: object_id }.to_string()
                        }
                    } else {
                        Avatar {
                            avatar_size: AvatarSize::ExtraLarge,
                            avatar_type: AvatarType::User
                        }
                    }
                }
                h2 {
                    class: "mt-2 text-center text-xl font-semibold",
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
                if has_some_examples {
                    h2 {
                        class: "mt-12 mb-8 text-xl font-semibold",
                        "Conversation Starters"
                    }
                    div {
                        class: "grid grid-cols-2 gap-x-1.5 gap-y-2",
                        for example in examples {
                            if let Some(example) = example {
                                if ! example.is_empty() {
                                    ExampleForm {
                                        team_id,
                                        prompt_id: prompt.id,
                                        example: example
                                    }
                                }
                            }
                        }
                    }
                }
                ModalAction {
                    class: "flex flex-row",
                    a {
                        class: "basis-3/4 btn btn-primary btn-sm",
                        href: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                        "Start a Chat"
                    }
                    Button {
                        class: "basis-1/4 cancel-modal",
                        button_type: ButtonType::Reset,
                        button_scheme: ButtonScheme::Error,
                        "Cancel"
                    }
                }
            }
        }
    }
}
