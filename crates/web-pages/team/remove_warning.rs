#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RemoveMemberWarningDrawer(trigger_id: String) -> Element {
    rsx! {
        Modal {
            trigger_id: &trigger_id,
            ModalBody {
                h3 {
                    class: "font-bold text-lg mb-4",
                    "A vault must have at least one admin user"
                }
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
        }
    }
}
