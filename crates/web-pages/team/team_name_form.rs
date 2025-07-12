#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn TeamNameForm(submit_action: String, trigger_id: String) -> Element {
    rsx! {
        form {
            method: "post",
            "data-turbo-frame": "_top",
            action: "{submit_action}",
            Modal {
                trigger_id: &trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Set Team Name"
                    }
                    div {
                        class: "flex flex-col",
                        Fieldset {
                            legend: "Name",
                            help_text: "Give your new team a name",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Team Name",
                                required: true,
                                name: "name"
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            button_size: ButtonSize::Small,
                            "Cancel"
                        }
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
