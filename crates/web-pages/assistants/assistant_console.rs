#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

use crate::console::empty_stream::EmptyStream;
use crate::console::{ChatWithChunks, PendingChatState};

#[component]
pub fn AssistantConsole(
    team_id: i32,
    conversation_id: Option<i64>,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    prompt: SinglePrompt,
    selected_item: SideBar,
    title: String,
    header: Element,
    is_tts_disabled: bool,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<BionicToolDefinition>,
) -> Element {
    let has_pending_chat = !matches!(&pending_chat_state, PendingChatState::None);

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
                if ! chat_history.is_empty() || has_pending_chat {
                    crate::console::console_stream::ConsoleStream {
                        team_id: team_id,
                        chat_history,
                        pending_chat_state: pending_chat_state.clone(),
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
                        lock_console: has_pending_chat,
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
