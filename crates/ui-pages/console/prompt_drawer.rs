#![allow(non_snake_case)]
use super::ChatChunks;
use dioxus::prelude::*;
use daisy_rsx::*;

#[component]
pub fn PromptDrawer(
    prompt: String,
    trigger_id: String,
    chunks: Vec<ChatChunks>,
) -> Element {
    rsx! {
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
