#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps {
    trigger_id: String,
}

pub fn HistoryDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "Your History",
            trigger_id: &cx.props.trigger_id,
            DrawerBody {
            }
            DrawerFooter {
            }
        }
    })
}
