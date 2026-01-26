#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn AssistantForm(team_id: String, prompts: Vec<Prompt>) -> Element {
    rsx!(
        form {
            action: crate::routes::api_keys::New{ team_id: team_id.clone() }.to_string(),
            method: "post",
            Modal {
                trigger_id: "create-assistant-key",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "New Assistant API Key"
                    }
                    div {
                        class: "flex flex-col",
                        Fieldset {
                            legend: "Name",
                            help_text: "Give your new key a name",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Production API Key",
                                required: true,
                                name: "name"
                            }
                        }
                        Fieldset {
                            legend: "Please select an Assistant",
                            legend_class: "mt-4",
                            help_text: "All access via this API key will use the above assistant",
                            Select {
                                name: "prompt_id",
                                {prompts.iter().map(|prompt| rsx!(
                                    SelectOption {
                                        value: "{prompt.id}",
                                        "{prompt.name}"
                                    }
                                ))}
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create API Key"
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn ModelForm(team_id: String, prompts: Vec<Prompt>) -> Element {
    rsx!(
        form {
            action: crate::routes::api_keys::New{ team_id: team_id.clone() }.to_string(),
            method: "post",
            Modal {
                trigger_id: "create-model-key",
                ModalBody {
                    h3 {
                        class: "font-bold text-lg mb-4",
                        "New Model API Key"
                    }
                    div {
                        class: "flex flex-col",
                        Fieldset {
                            legend: "Name",
                            help_text: "Give your new key a name",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "Production API Key",
                                required: true,
                                name: "name"
                            }
                        }
                        Fieldset {
                            legend: "Please select a Model",
                            legend_class: "mt-4",
                            help_text: "All access via this API key will use the above model",
                            Select {
                                name: "prompt_id",
                                {prompts.iter().map(|prompt| rsx!(
                                    SelectOption {
                                        value: "{prompt.id}",
                                        "{prompt.name}"
                                    }
                                ))}
                            }
                        }
                    }
                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_scheme: ButtonScheme::Warning,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Create API Key"
                        }
                    }
                }
            }
        }
    )
}
