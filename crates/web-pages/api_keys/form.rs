#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn AssistantForm(team_id: i32, prompts: Vec<Prompt>) -> Element {
    rsx!(
        form {
            action: crate::routes::api_keys::New{ team_id }.to_string(),
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
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Production API Key",
                            help_text: "Give your new key a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "prompt_id",
                            label: "Please select an Assistant",
                            label_class: "mt-4",
                            help_text: "All access via this API key will use the above assistant",
                            {prompts.iter().map(|prompt| rsx!(
                                SelectOption {
                                    value: "{prompt.id}",
                                    "{prompt.name}"
                                }
                            ))}
                        }
                    }
                    ModalAction {
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
pub fn ModelForm(team_id: i32, prompts: Vec<Prompt>) -> Element {
    rsx!(
        form {
            action: crate::routes::api_keys::New{ team_id }.to_string(),
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
                        Input {
                            input_type: InputType::Text,
                            placeholder: "Production API Key",
                            help_text: "Give your new key a name",
                            required: true,
                            label: "Name",
                            name: "name"
                        }
                        Select {
                            name: "prompt_id",
                            label: "Please select a Model",
                            label_class: "mt-4",
                            help_text: "All access via this API key will use the above model",
                            {prompts.iter().map(|prompt| rsx!(
                                SelectOption {
                                    value: "{prompt.id}",
                                    "{prompt.name}"
                                }
                            ))}
                        }
                    }
                    ModalAction {
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
