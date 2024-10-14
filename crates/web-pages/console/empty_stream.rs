#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

#[component]
pub fn EmptyStream(prompt: SinglePrompt, conversation_id: i64, team_id: i32) -> Element {
    rsx! {
        div {
            class: "flex h-[calc(100%-100px)] overflow-y-auto justify-center items-center",
            div {
                class: "mx-3 mt-12 max-w-3xl gap-4 text-center",
                img {
                    width: "47",
                    height: "47",
                    class: "svg-icon mb-8 mx-auto",
                    src: ai_svg.name
                }
                div {
                    class: "flex flex-nowrap max-w-3xl flex-wrap items-stretch justify-center gap-4",
                    if let Some(example1) = prompt.example1 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example1
                        }
                    }
                    if let Some(example2) = prompt.example2 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example2
                        }
                    }
                    if let Some(example3) = prompt.example3 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example3
                        }
                    }
                    if let Some(example4) = prompt.example4 {
                        ExampleForm {
                            conversation_id,
                            team_id,
                            prompt_id: prompt.id,
                            example: example4
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ExampleForm(conversation_id: i64, prompt_id: i32, team_id: i32, example: String) -> Element {
    rsx! {
        form {
            class: "flex h-full",
            method: "post",
            action: routes::console::SendMessage{team_id}.to_string(),
            input {
                "type": "hidden",
                name: "conversation_id",
                value: "{conversation_id}"
            }
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
                class: "flex flex-grow h-full flex-col gap-2 rounded-2xl border border-token-border-light px-3 pb-4 pt-3 text-start align-top text-[15px] shadow-xxs transition",
                "type": "submit",
                img {
                    height: "16",
                    width: "16",
                    class: "svg-icon mr-6",
                    src: ai_svg.name
                }
                "{example}"
            }
        }
    }
}
