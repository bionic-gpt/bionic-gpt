#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn Form() -> Element {
    rsx!(
        Drawer {
            label: "Enter Licence",
            trigger_id: "create-licence",
            DrawerBody {
                div {
                }
            }
            DrawerFooter {
            }
        }
    )
}
