#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

use crate::console::{ChatWithChunks, PendingChat};
use crate::routes;

#[component]
pub fn AssistantConsole(
    team_id: i32,
    conversation_id: Option<i64>,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat: Option<PendingChat>,
    prompt: SinglePrompt,
    selected_item: SideBar,
    title: String,
    header: Element,
    is_tts_disabled: bool,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<(String, String)>,
) -> Element {
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item,
            team_id: team_id,
            rbac: rbac.clone(),
            title,
            header,
            div {
                id: "console-panel",
                class: "h-full flex flex-col",
                if ! chat_history.is_empty() {
                    crate::console::console_stream::ConsoleStream {
                        team_id: team_id,
                        chat_history,
                        pending_chat: pending_chat.clone(),
                        is_tts_disabled,
                        rbac: rbac.clone()
                    }
                } else {
                    div {
                        class: "flex-1 flex flex-col justify-center h-full",
                        EmptyStream {
                            prompt: prompt.clone(),
                            team_id
                        },
                    }
                }
                div {
                    crate::console::prompt_form::Form {
                        team_id: team_id,
                        prompt_id: prompt.id,
                        lock_console: pending_chat.is_some(),
                        conversation_id,
                        disclaimer: prompt.disclaimer,
                        capabilities,
                        enabled_tools,
                        available_tools,
                    },
                }
            }
        }
    }
}

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
pub fn ExampleForm(prompt_id: i32, team_id: i32, example: String) -> Element {
    rsx! {
        form {
            class: "w-full",
            method: "post",
            action: routes::console::SendMessage{team_id}.to_string(),
            input {
                class: "set-my-prompt-id",
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
