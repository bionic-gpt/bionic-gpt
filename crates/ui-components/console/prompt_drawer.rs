#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps {
    prompt: String,
    trigger_id: String,
}

pub fn PromptDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Full Prompt",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
                pre {
                    style: "white-space: pre-wrap",
                    "{cx.props.prompt}"
                }
            }
            DrawerFooter {
            }
        }
    })
}
