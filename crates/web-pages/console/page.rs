#![allow(non_snake_case)]
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;
use openai_api::BionicToolDefinition;

pub fn new_conversation(
    team_id: String,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
    rbac: Rbac,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<BionicToolDefinition>,
) -> String {
    // Rerverse it because that's how we display it.
    crate::render(rsx! {
        super::layout::ConsoleLayout {
            team_id: team_id.clone(),
            rbac: rbac.clone(),
            prompt: prompt.clone(),
            title: "AI Chat Console",
            selected_item: SideBar::Console,
            chat_history: vec![],
            pending_chat_state: super::PendingChatState::None,
            is_tts_disabled: true,
            capabilities,
            enabled_tools,
            available_tools,
            header: rsx!(
                Head {
                    team_id: team_id.clone(),
                    rbac: rbac.clone(),
                    prompts,
                    prompt: prompt.clone()
                }
            )
        }
    })
}

#[component]
fn Head(team_id: String, rbac: Rbac, prompts: Vec<Prompt>, prompt: SinglePrompt) -> Element {
    rsx! {
        ModelPopup {
            id: prompt.id,
            value: prompt.name,
            prompts
        }
    }
}
