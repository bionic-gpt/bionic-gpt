#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn PromptDrawer(cx: Scope, prompt: String, trigger_id: String) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Full Prompt",
            trigger_id: &trigger_id,
            DrawerBody {
                pre {
                    "{prompt}"
                }
            }
            DrawerFooter {
            }
        }
    })
}
