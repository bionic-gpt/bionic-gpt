#![allow(non_snake_case)]
use super::ChatChunks;
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn PromptDrawer(
    cx: Scope,
    prompt: String,
    trigger_id: String,
    chunks: Vec<ChatChunks>,
) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Full Prompt",
            trigger_id: &trigger_id,
            DrawerBody {
                class: "prose prose-sm",
                pre {
                    class: "json",
                    "{prompt}"
                }

                if ! chunks.is_empty() {
                    cx.render(rsx!(
                        h4 {
                            "Context provided to the prompt from your documents"
                        }

                        ol {
                            chunks.iter().map(|chunk| {
                                cx.render(rsx!(
                                    li {
                                        "A section from Page {chunk.page_number} in file {chunk.file_name}."
                                    }
                                ))
                            })
                        }
                    ))
                }
            }
            DrawerFooter {
            }
        }
    })
}
