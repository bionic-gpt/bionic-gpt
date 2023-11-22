#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct DrawerProps {
    trigger_id: String,
}

pub fn RemoveMemberWarningDrawer(cx: Scope<DrawerProps>) -> Element {
    cx.render(rsx! {
        Drawer {
            label: "A vault must have at least one admin user",
            trigger_id: &cx.props.trigger_id,
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
