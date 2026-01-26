#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn ApiKeyForm(team_id: String, integration_id: i32, integration_name: String) -> Element {
    let trigger_id = format!("configure-api-key-{}", integration_id);

    rsx!(
        form {
            action: crate::routes::integrations::ConfigureApiKey { team_id: team_id.clone(), integration_id }.to_string(),
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
                        Fieldset {
                            legend: "API Key",
                            help_text: "This API key will be used to authenticate requests to the integration",
                            Input {
                                input_type: InputType::Password,
                                placeholder: "Enter your API key",
                                required: true,
                                name: "api_key"
                            }
                        }
                        Fieldset {
                            legend: "Visibility",
                            legend_class: "mt-4",
                            help_text: "Who can use this API key connection",
                            Select {
                                name: "visibility",
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
