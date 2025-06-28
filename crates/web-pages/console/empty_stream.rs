#![allow(non_snake_case)]
use assets::files::*;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

#[component]
pub fn EmptyStream(prompt: SinglePrompt, conversation_id: Option<i64>, team_id: i32) -> Element {
    let examples: Vec<Option<String>> = vec![
        prompt.example1,
        prompt.example2,
        prompt.example3,
        prompt.example4,
    ];
    rsx! {
        div {
            class: "mx-auto mt-12 max-w-3xl text-center",
            h1 {
                class: "mb-8 text-2xl font-semibold relative",
                "What can I help with?"
            }
            div {
                class: "grid grid-cols-2 md:grid-cols-4 pl-2 pr-2 max-w-3xl flex-wrap items-stretch justify-center gap-4",
                for example in examples {
                    if let Some(example) = example {
                        if ! example.is_empty() {
                            ExampleForm {
                                example
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ExampleForm(example: String) -> Element {
    rsx! {
        button {
            class: "example-prompt cursor-pointer hover:bg-base-200 flex flex-col h-full w-full rounded-2xl border p-3 text-start",
            "type": "button",
            "data-example": "{example}",
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
