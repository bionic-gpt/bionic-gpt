#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
use super::{ChatWithChunks, PendingChatState};
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use crate::ConfirmModal;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat_state: PendingChatState,
    prompts: Vec<Prompt>,
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
        super::layout::ConsoleLayout {
            team_id: team_id,
            rbac: rbac.clone(),
            title: "AI Chat Console",
            prompt: prompt.clone(),
            selected_item: SideBar::Console,
            chat_history,
            pending_chat_state,
            conversation_id,
            is_tts_disabled,
            capabilities,
            enabled_tools,
            available_tools,
            header: rsx!(
                Head {
                    team_id: team_id,
                    rbac: rbac.clone(),
                    conversation_id: conversation_id,
                    prompts,
                    prompt: prompt.clone()
                }
            )
        }
    };

    crate::render(page)
}

#[component]
fn Head(
    team_id: i32,
    rbac: Rbac,
    conversation_id: i64,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
) -> Element {
    rsx! {

        ModelPopup {
            id: prompt.id,
            value: prompt.name,
            prompts
        }
        div {
            class: "flex flex-row",
            if rbac.can_delete_chat() {
                Button {
                    class: "btn-circle mr-2 p-1",
                    popover_target: "delete-conv-{conversation_id}",
                    button_scheme: ButtonScheme::Neutral,
                    img {
                        class: "svg-icon",
                        src: delete_svg.name
                    }
                }
                ConfirmModal {
                    action: crate::routes::console::Delete{team_id, id: conversation_id}.to_string(),
                    trigger_id: format!("delete-conv-{}", conversation_id),
                    submit_label: "Delete".to_string(),
                    heading: "Delete this Conversation?".to_string(),
                    warning: "Are you sure you want to delete this Conversation?".to_string(),
                    hidden_fields: vec![
                        ("team_id".into(), team_id.to_string()),
                        ("id".into(), conversation_id.to_string()),
                    ],
                }
            }
            a {
                href: crate::routes::console::Index{team_id}.to_string(),
                class: "btn btn-primary btn-sm mr-4",
                "New Chat"
            }
        }
    }
}
