#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
use super::{ChatWithChunks, PendingChat};
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat: Option<PendingChat>,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
    conversation_id: i64,
    is_tts_disabled: bool,
    capabilities: Vec<Capability>,
    enabled_tools: Vec<String>,
    available_tools: Vec<(String, String)>,
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
            pending_chat,
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
                    drawer_trigger: "delete-conv-{conversation_id}",
                    button_scheme: ButtonScheme::Default,
                    img {
                        class: "svg-icon",
                        src: delete_svg.name
                    }
                }
                super::delete::DeleteDrawer{
                    trigger_id: format!("delete-conv-{}", conversation_id),
                    team_id: team_id,
                    id: conversation_id
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
