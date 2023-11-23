#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn PromptDrawer(cx: Scope, prompt: String, trigger_id: String) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Full Prompt",
            trigger_id: &trigger_id,
            DrawerBody {
                pre {
                    style: "white-space: pre-wrap",
                    "{prompt}"
                }
            }
            DrawerFooter {
            }
        }
    })
}
