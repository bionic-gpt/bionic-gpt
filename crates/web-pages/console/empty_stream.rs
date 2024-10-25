#![allow(non_snake_case)]
use crate::routes;

use assets::files::*;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

#[component]
pub fn EmptyStream(prompt: SinglePrompt, conversation_id: Option<i64>, team_id: i32) -> Element {
    rsx! {
        div {
            class: "flex h-[calc(100%-100px)] overflow-y-auto justify-center items-center",
            div {
                class: "mx-3 mt-12 max-w-3xl gap-4 text-center",
                h1 {
                    class: "mb-8 text-2xl font-semibold relative before:absolute before:inset-0 before:animate-typewriter before:bg-white",
                    "What can I help with?"
                }
                div {
                    class: "flex flex-nowrap max-w-3xl flex-wrap items-stretch justify-center gap-4",
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
        }
    }
}

#[component]
pub fn ExampleForm(conversation_id: i64, prompt_id: i32, team_id: i32, example: String) -> Element {
    rsx! {
        form {
            class: "w-full",
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
                class: "flex flex-col h-full w-full rounded-2xl border p-3 text-start",
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
