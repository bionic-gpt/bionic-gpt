#![allow(non_snake_case)]
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use db::authz::Rbac;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;

#[component]
pub fn NewConversation(
    team_id: i32,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
    rbac: Rbac,
) -> Element {
    // Rerverse it because that's how we display it.
    rsx! {
        super::layout::ConsoleLayout {
            team_id,
            rbac: rbac.clone(),
            chats_with_chunks: vec![],
            prompt: prompt.clone(),
            title: "AI Chat Console",
            selected_item: SideBar::Console,
            header: rsx!(
                Head {
                    team_id: team_id,
                    rbac: rbac.clone(),
                    prompts,
                    prompt: prompt.clone()
                }
            )
        }
    }
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
