#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn ToolsModal() -> Element {
    rsx!(
        form {
            action: "",
            method: "post",
            Modal {
                trigger_id: "tool-modal",
                ModalBody {

                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Save"
                        }
                    }
                }
            }
        }
    )
}
