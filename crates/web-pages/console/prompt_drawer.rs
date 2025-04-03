#![allow(non_snake_case)]
use super::ChatChunks;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn PromptDrawer(
    prompt: String,
    trigger_id: String,
    chunks: Vec<ChatChunks>,
    rbac: Rbac,
) -> Element {
    rsx! {
        Drawer {
            label: "Full Prompt",
            trigger_id: &trigger_id,
            DrawerBody {
                class: "prose prose-sm",
                if rbac.can_view_system_prompt() {
                    pre {
                        class: "json",
                        "{prompt}"
                    }
                } else {
                    div {
                        class: "alert alert-warning",
                        "You need permission to view the system prompt"
                    }
                }

                if ! chunks.is_empty() {
                    h4 {
                        "Context provided to the prompt from your documents"
                    }

                    ol {
                        for chunk in chunks {
                            li {
                                "A section from Page {chunk.page_number} in file {chunk.file_name}."
                            }
                        }
                    }
                }
            }
            DrawerFooter {
            }
        }
    }
}
