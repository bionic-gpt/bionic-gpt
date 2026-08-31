#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::models::ModelConfig;
use dioxus::prelude::*;

use super::{ChatWithChunks, PendingChatState};

#[component]
pub fn ConsoleLayout(
    team_id: String,
    conversation_id: Option<i64>,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    prompt: ModelConfig,
    selected_item: SideBar,
    title: String,
    header: Element,
    is_tts_disabled: bool,
    capabilities: Vec<Capability>,
    #[props(default)] selected_project_id: Option<i32>,
) -> Element {
    let has_pending_chat = pending_chat_state.shall_we_call_the_model();

    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item,
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            title,
            header,
            selected_project_id,
            div {
                id: "console-panel",
                class: "h-full flex flex-col",
                if ! chat_history.is_empty() || has_pending_chat {
                    super::console_stream::ConsoleStream {
                        team_id: team_id.clone(),
                        chat_history,
                        pending_chat_state: pending_chat_state.clone(),
                        is_tts_disabled,
                        rbac: rbac.clone()
                    }
                    div {
                        super::prompt_form::Form {
                            team_id: team_id.clone(),
                            model_id: prompt.id,
                            lock_console: has_pending_chat,
                            conversation_id,
                            disclaimer: prompt.disclaimer,
                            capabilities: capabilities.clone(),
                        },
                    }
                } else {
                    div {
                        class: "flex-1 flex flex-col justify-center h-full",
                        h1 {
                            class: "mx-auto mb-8 text-2xl font-semibold relative",
                            "What can I help with?"
                        }
                        div {
                            super::prompt_form::Form {
                                team_id: team_id.clone(),
                                model_id: prompt.id,
                                lock_console: has_pending_chat,
                                conversation_id,
                                disclaimer: prompt.disclaimer,
                                capabilities,
                            },
                        }
                    }
                }
            }
        }
    }
}
