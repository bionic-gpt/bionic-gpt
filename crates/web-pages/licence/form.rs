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
                Alert {
                    alert_color: AlertColor::Info,
                    class: "mb-3",
                    p {
                        "Please contact us to arrange licencing."
                    }
                }
            }
            DrawerFooter {
            }
        }
    )
}
