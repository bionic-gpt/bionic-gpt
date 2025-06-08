#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn ApiKeyForm(team_id: i32, integration_id: i32, integration_name: String) -> Element {
    let trigger_id = format!("configure-api-key-{}", integration_id);

    rsx!(
        form {
            action: crate::routes::integrations::ConfigureApiKey { team_id, integration_id }.to_string(),
            method: "post",
            Modal {
                trigger_id: trigger_id,
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "Configure API Key for {integration_name}"
                    }
                    div {
                        class: "flex flex-col",
                        Input {
                            input_type: InputType::Password,
                            placeholder: "Enter your API key",
                            help_text: "This API key will be used to authenticate requests to the integration",
                            required: true,
                            label: "API Key",
                            name: "api_key"
                        }
                        Select {
                            name: "visibility",
                            label: "Visibility",
                            label_class: "mt-4",
                            help_text: "Who can use this API key connection",
                            SelectOption {
                                value: "Private",
                                "Private (Only you)"
                            }
                            SelectOption {
                                value: "Team",
                                "Team (All team members)"
                            }
                        }
                    }
                    ModalAction {
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
