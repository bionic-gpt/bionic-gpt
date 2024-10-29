#![allow(non_snake_case)]
use super::ChatWithChunks;
use crate::app_layout::SideBar;
use crate::console::model_popup::ModelPopup;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::prompts::{Prompt, SinglePrompt};
use dioxus::prelude::*;

#[component]
pub fn Conversation(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompts: Vec<Prompt>,
    prompt: SinglePrompt,
    conversation_id: i64,
    lock_console: bool,
    is_tts_disabled: bool,
) -> Element {
    // Rerverse it because that's how we display it.
    let chats_with_chunks: Vec<ChatWithChunks> = chats_with_chunks.into_iter().rev().collect();
    rsx! {
        super::layout::ConsoleLayout {
            team_id: team_id,
            rbac: rbac.clone(),
            title: "AI Chat Console",
            prompt: prompt.clone(),
            selected_item: SideBar::Console,
            chats_with_chunks,
            lock_console,
            is_tts_disabled,
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
    }
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
