#![allow(non_snake_case)]
use super::ChatWithChunks;
use crate::app_layout::{Layout, SideBar};
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
                    conversation_id: conversation_id,
                    prompts,
                    prompt: prompt.clone()
                }
            ),
            div {
                id: "console-panel",
                class: "h-full",
                if chats_with_chunks.is_empty() {
                    crate::console::empty_stream::EmptyStream {
                        prompt: prompt.clone(),
                        conversation_id,
                        team_id
                    }
                } else {
                    super::console_stream::ConsoleStream {
                        team_id: team_id,
                        chats_with_chunks: chats_with_chunks,
                        is_tts_disabled: is_tts_disabled,
                        lock_console: lock_console,
                    }
                }
                super::prompt_form::Form {
                    team_id: team_id,
                    prompt_id: prompt.id,
                    conversation_id: conversation_id,
                    lock_console: lock_console,
                    disclaimer: prompt.disclaimer
                }
            }
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
            form {
                method: "post",
                action: crate::routes::console::NewChat{team_id}.to_string(),
                Button {
                    class: "mr-2",
                    button_scheme: ButtonScheme::Default,
                    button_type: ButtonType::Submit,
                    "New Chat"
                }
            }
        }
    }
}
