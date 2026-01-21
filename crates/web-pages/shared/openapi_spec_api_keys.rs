#![allow(non_snake_case)]
use daisy_rsx::*;
use db::OpenapiSpec;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OpenapiSpecKeySummary {
    pub spec: OpenapiSpec,
    pub has_api_key: bool,
    pub has_key_configured: bool,
}

#[component]
pub fn OpenapiSpecApiKeyForm(trigger_id: String, action: String, spec_title: String) -> Element {
    rsx!(
        form {
            action: "{action}",
            method: "post",
            Modal {
                trigger_id: trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Configure API Key for {spec_title}"
                    }
                    div {
                        class: "flex flex-col",
                        Fieldset {
                            legend: "API Key",
                            help_text: "This overwrites the current key used for this spec.",
                            Input {
                                input_type: InputType::Password,
                                placeholder: "Enter the API key",
                                required: true,
                                name: "api_key"
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
                            "Save API Key"
                        }
                    }
                }
            }
        }
    )
}
