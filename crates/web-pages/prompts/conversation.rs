#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::console::ChatWithChunks;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{conversations::History, prompts::SinglePrompt};
use dioxus::prelude::*;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompt: SinglePrompt,
    conversation_id: i64,
    history: Vec<History>,
    lock_console: bool,
    is_tts_disabled: bool,
) -> Element {
    // Rerverse it because that's how we display it.
    let chats_with_chunks: Vec<ChatWithChunks> = chats_with_chunks.into_iter().rev().collect();
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac,
            title: "{prompt.name}",
            header: rsx!(
                h3 { "{prompt.name}" }
                if ! chats_with_chunks.is_empty() {
                    div {
                        class: "flex flex-row",
                        Button {
                            class: "btn-circle mr-2 p-1",
                            drawer_trigger: "delete-conv-{conversation_id}",
                            button_scheme: ButtonScheme::Default,
                            img {
                                src: delete_svg.name
                            }
                        }
                        super::delete_conv::DeleteDrawer{
                            trigger_id: format!("delete-conv-{}", conversation_id),
                            team_id: team_id,
                            prompt_id: prompt.id,
                            conversation_id
                        }
                        form {
                            method: "get",
                            action: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                            Button {
                                class: "mr-2",
                                button_scheme: ButtonScheme::Default,
                                button_type: ButtonType::Submit,
                                "New Chat"
                            }
                        }
                    }
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                if chats_with_chunks.is_empty() {
                    crate::console::empty_stream::EmptyStream {
                        prompt: prompt.clone(),
                        conversation_id,
                        team_id
                    }
                } else {
                    crate::console::console_panel::ConsolePanel {
                        chats_with_chunks,
                        team_id,
                        is_tts_disabled: is_tts_disabled,
                        lock_console: lock_console,
                    }
                }

                crate::console::prompt_form::Form {
                    team_id: team_id,
                    prompt_id: prompt.id,
                    conversation_id: conversation_id,
                    lock_console: lock_console,
                    disclaimer: prompt.disclaimer
                }
            }
        }
    }
}
