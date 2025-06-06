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
        Modal {
            trigger_id: &trigger_id,
            ModalBody {
                class: "prose prose-sm max-w-4xl",
                h3 {
                    class: "font-bold text-lg mb-4",
                    "Full Prompt"
                }
                if rbac.can_view_system_prompt() {
                    pre {
                        class: "json bg-gray-100 p-4 rounded overflow-auto max-h-96",
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
                        class: "mt-6 mb-3",
                        "Context provided to the prompt from your documents"
                    }

                    ol {
                        class: "space-y-1",
                        for chunk in chunks {
                            li {
                                "A section from Page {chunk.page_number} in file {chunk.file_name}."
                            }
                        }
                    }
                }
            }
        }
    }
}
