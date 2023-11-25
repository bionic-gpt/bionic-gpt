#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn RemoveMemberWarningDrawer(cx: Scope, trigger_id: String) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "A vault must have at least one admin user",
            trigger_id: &trigger_id,
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Alert {
                        alert_color: AlertColor::Warn,
                        class: "mb-3",
                        h4 {
                            "A vault must have at least one user who is an administrator."
                        }
                    }
                }
            }
            DrawerFooter {
            }
        }
    })
}
