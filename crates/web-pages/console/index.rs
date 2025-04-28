#![allow(non_snake_case)]
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;

pub fn new_conversation(
    team_id: i32,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
    rbac: Rbac,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
) -> String {
    // Rerverse it because that's how we display it.
    crate::render(rsx! {
        super::layout::ConsoleLayout {
            team_id,
            rbac: rbac.clone(),
            prompt: prompt.clone(),
            title: "AI Chat Console",
            selected_item: SideBar::Console,
            chats_with_chunks: vec![],
            is_tts_disabled: true,
            lock_console: false,
            capabilities,
            enabled_tools,
            header: rsx!(
                Head {
                    team_id: team_id,
                    rbac: rbac.clone(),
                    prompts,
                    prompt: prompt.clone()
                }
            )
        }
    })
}

#[component]
fn Head(team_id: i32, rbac: Rbac, prompts: Vec<Prompt>, prompt: SinglePrompt) -> Element {
    rsx! {
        ModelPopup {
            id: prompt.id,
            value: prompt.name,
            prompts
        }
    }
}
