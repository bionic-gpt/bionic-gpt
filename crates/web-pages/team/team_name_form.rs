#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn TeamNameForm(submit_action: String) -> Element {
    rsx! {
        form {
            method: "post",
            "data-turbo-frame": "_top",
            action: "{submit_action}",
            Modal {
                trigger_id: "set-name-drawer",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Set Team Name"
                    }
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Team Name",
                            help_text: "Give your new team a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Set Team Name"
                        }
                    }
                }
            }
        }
    }
}
