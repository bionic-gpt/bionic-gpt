#![allow(non_snake_case)]
use crate::routes::prompts::Image;
use assets::files::*;
use daisy_rsx::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;

#[component]
pub fn ViewDrawer(team_id: String, prompt: Prompt, trigger_id: String) -> Element {
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
                            width: "48",
                            height: "48",
                            src: Image { team_id: team_id.clone(), id: object_id }.to_string()
                        }
                    } else {
                        Avatar {
                            avatar_size: AvatarSize::Medium,
                            name: "{prompt.name}"
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
                                        prompt_id: prompt.id,
                                        team_id: team_id.clone(),
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
                        href: crate::routes::prompts::NewChat{team_id: team_id.clone(), prompt_id: prompt.id}.to_string(),
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

#[component]
pub fn ExampleForm(prompt_id: i32, team_id: String, example: String) -> Element {
    rsx! {
        form {
            class: "w-full",
            method: "post",
            action: crate::routes::console::SendMessage{team_id: team_id.clone()}.to_string(),
            enctype: "multipart/form-data",
            input {
                "type": "hidden",
                name: "prompt_id",
                value: "{prompt_id}"
            }
            input {
                "type": "hidden",
                name: "message",
                value: "{example}"
            }
            button {
                class: "cursor-pointer hover:bg-base-200 flex flex-col h-full w-full rounded-2xl border p-3 text-start",
                "type": "submit",
                img {
                    height: "16",
                    width: "16",
                    class: "svg-icon mb-2",
                    src: ai_svg.name
                }
                "{example}"
            }
        }
    }
}
