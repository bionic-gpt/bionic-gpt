#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn SystemPromptCard(system_prompt: String) -> Element {
    rsx! {
        Card {
            class: "mb-6",
            CardHeader {
                title: "System Prompt"
            }
            CardBody {
                div {
                    class: "bg-gray-50 p-4 rounded border font-mono text-sm leading-relaxed whitespace-pre-wrap",
                    "{system_prompt}"
                }
            }
        }
    }
}
