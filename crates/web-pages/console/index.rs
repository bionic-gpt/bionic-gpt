#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
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
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item: SideBar::Console,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "AI Chat Console",
            header: rsx!(
                Head {
                    team_id: team_id,
                    rbac: rbac.clone(),
                    prompts,
                    prompt: prompt.clone()
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                crate::console::empty_stream::EmptyStream {
                    prompt: prompt.clone(),
                    team_id
                }
                super::prompt_form::Form {
                    team_id: team_id,
                    prompt_id: prompt.id,
                    lock_console: false,
                    disclaimer: prompt.disclaimer
                }
            }
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
