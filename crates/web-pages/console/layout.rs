#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use db::authz::Rbac;
use db::queries::prompts::SinglePrompt;
use dioxus::prelude::*;

use super::ChatWithChunks;

#[component]
pub fn ConsoleLayout(
    team_id: i32,
    rbac: Rbac,
    chats_with_chunks: Vec<ChatWithChunks>,
    prompt: SinglePrompt,
    selected_item: SideBar,
    title: String,
    header: Element,
) -> Element {
    // Rerverse it because that's how we display it.
    rsx! {
        Layout {
            section_class: "console flex flex-col justify-start h-[calc(100%-79px)]",
            selected_item,
            team_id: team_id,
            rbac: rbac.clone(),
            title,
            header,
            div {
                id: "console-panel",
                class: "h-full flex flex-col",
                div {
                    class: "flex-1 flex flex-col justify-center h-full",
                    crate::console::empty_stream::EmptyStream {
                        prompt: prompt.clone(),
                        team_id
                    }
                }
                div {
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
}
