#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
use super::assistant_console::AssistantConsole;
use crate::app_layout::SideBar;
use crate::console::{ChatWithChunks, PendingChatState};
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    prompt: SinglePrompt,
    conversation_id: i64,
    is_tts_disabled: bool,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<BionicToolDefinition>,
) -> String {
    // Rerverse it because that's how we display it.
    let chat_history: Vec<ChatWithChunks> = chat_history.into_iter().rev().collect();
    let page = rsx! {
        AssistantConsole {
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac,
            title: "{prompt.name}",
            chat_history: chat_history.clone(),
            pending_chat_state,
            prompt: prompt.clone(),
            conversation_id,
            is_tts_disabled,
            capabilities,
            enabled_tools,
            available_tools,
            header: rsx!(
                h3 { "{prompt.name}" }
                if ! chat_history.is_empty() {
                    div {
                        class: "flex flex-row",
                        Button {
                            class: "btn-circle mr-2 p-1",
                            popover_target: "delete-conv-{conversation_id}",
                            button_scheme: ButtonScheme::Neutral,
                            img {
                                src: delete_svg.name
                            }
                        }
                        ConfirmModal {
                            action: crate::routes::prompts::DeleteConv{team_id, prompt_id: prompt.id, conversation_id}.to_string(),
                            trigger_id: format!("delete-conv-{}", conversation_id),
                            submit_label: "Delete".to_string(),
                            heading: "Delete this Conversation?".to_string(),
                            warning: "Are you sure you want to delete this Conversation?".to_string(),
                            hidden_fields: vec![
                                ("team_id".into(), team_id.to_string()),
                                ("id".into(), conversation_id.to_string()),
                                ("prompt_id".into(), prompt.id.to_string()),
                            ],
                        }
                        form {
                            method: "get",
                            action: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                            Button {
                                class: "mr-2",
                                button_scheme: ButtonScheme::Neutral,
                                button_type: ButtonType::Submit,
                                "New Chat"
                            }
                        }
                    }
                }
            )
        }
    };

    crate::render(page)
}
