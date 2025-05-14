#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
use super::assistant_console::AssistantConsole;
use crate::app_layout::SideBar;
use crate::console::{ChatWithChunks, PendingChat};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::capabilities::Capability;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    chat_history: Vec<ChatWithChunks>,
    pending_chat: Option<PendingChat>,
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
        AssistantConsole {
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac,
            title: "{prompt.name}",
            chat_history: chat_history.clone(),
            pending_chat,
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
                            drawer_trigger: "delete-conv-{conversation_id}",
                            button_scheme: ButtonScheme::Default,
                            img {
                                src: delete_svg.name
                            }
                        }
                        super::delete_conv::DeleteDrawer{
                            trigger_id: format!("delete-conv-{}", conversation_id),
                            team_id: team_id,
                            prompt_id: prompt.id,
                            conversation_id
                        }
                        form {
                            method: "get",
                            action: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                            Button {
                                class: "mr-2",
                                button_scheme: ButtonScheme::Default,
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
