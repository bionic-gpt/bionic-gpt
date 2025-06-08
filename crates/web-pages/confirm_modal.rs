#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConfirmModalProps {
    action: String,
    trigger_id: String,
    submit_label: String,
    heading: String,
    warning: String,
    hidden_fields: Vec<(String, String)>,
}

#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    let hidden_fields = props.hidden_fields.clone();
    rsx! {
        form {
            action: props.action,
            method: "post",
            Modal {
                trigger_id: props.trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "{props.heading}"
                    }
                    div {
                        class: "flex flex-col",
                        Alert {
                            alert_color: AlertColor::Warn,
                            class: "mb-3",
                            p { "{props.warning}" }
                        }
                        for (name, value) in hidden_fields {
                            input {
                                "type": "hidden",
                                name: "{name}",
                                value: "{value}"
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Error,
                            "{props.submit_label}"
                        }
                    }
                }
            }
        }
    }
}
