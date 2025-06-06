#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use dioxus::prelude::*;

#[component]
pub fn Form(
    team_id: i32,
    name: String,
    display_name: String,
    base_url: String,
    model_type: String,
    trigger_id: String,
    api_key: String,
    tpm_limit: i32,
    rpm_limit: i32,
    context_size_bytes: i32,
    id: Option<i32>,
    prompt_id: Option<i32>,
    disclaimer: String,
    description: String,
    example1: String,
    example2: String,
    example3: String,
    example4: String,
    // Add new parameters for capabilities
    has_capability_function_calling: bool,
    has_capability_vision: bool,
    has_capability_tool_use: bool,
) -> Element {
    rsx!(
        form {
            action: crate::routes::models::New{team_id}.to_string(),
            method: "post",
            Modal {
                trigger_id: "{trigger_id}",
                ModalBody {
                    class: "flex flex-col justify-between md:w-full max-w-5xl h-full",
                    TabContainer {
                        TabPanel {
                            checked: true,
                            name: "prompt-tabs",
                            tab_name: "Assistant",
                            div {
                                class: "flex flex-col mt-3",
                                if let Some(id) = id {
                                    input {
                                        "type": "hidden",
                                        value: "{id}",
                                        name: "id"
                                    }
                                }
                                if let Some(id) = prompt_id {
                                    input {
                                        "type": "hidden",
                                        value: "{id}",
                                        name: "prompt_id"
                                    }
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "display_name",
                                    label: "Display Name",
                                    help_text: "Make the name memorable and imply it's usage.",
                                    value: display_name,
                                    required: true
                                }

                                TextArea {
                                    class: "mt-3",
                                    name: "description",
                                    rows: "8",
                                    label: "Description",
                                    help_text: "A brief summary about this model.",
                                    label_class: "mt-4",
                                    required: true,
                                    "{description}"
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "name",
                                    label: "Model Name",
                                    help_text: "The model's id as used in the API. i.e. llama3-70b",
                                    value: name,
                                    required: true
                                }

                                Select {
                                    name: "model_type",
                                    label: "Is this model for LLM or Embeddings",
                                    label_class: "mt-4",
                                    help_text: "Some models can do both, in which case enter it twice.",
                                    value: model_type.clone(),
                                    SelectOption {
                                        value: "LLM",
                                        selected_value: model_type.clone(),
                                        "Large Language Model"
                                    }
                                    SelectOption {
                                        value: "Embeddings",
                                        selected_value: model_type.clone(),
                                        "Embeddings Model"
                                    }
                                    SelectOption {
                                        value: "Image",
                                        selected_value: model_type.clone(),
                                        "Image Generation"
                                    }
                                    SelectOption {
                                        value: "TextToSpeech",
                                        selected_value: model_type.clone(),
                                        "Text To Speech"
                                    }
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "base_url",
                                    label: "The Base URL of the model",
                                    help_text: "The URL location of the OpenAI compatible API",
                                    value: base_url,
                                    required: true
                                }


                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "api_key",
                                    label: "The API secret from your provider",
                                    help_text: "This will be given in the providers console",
                                    value: api_key
                                }
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Advanced",
                            div {
                                class: "flex flex-col mt-3",

                                Input {
                                    input_type: InputType::Number,
                                    label_class: "mt-4",
                                    name: "tpm_limit",
                                    label: "Set the maximum tokens per minute for each user.",
                                    help_text: "If users exceed this limit there access to the model will be limited.",
                                    value: "{tpm_limit}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    label_class: "mt-4",
                                    name: "rpm_limit",
                                    label: "Set the maximum requests per minute for each user.",
                                    help_text: "If users exceed this limit there access to the model will be limited.",
                                    value: "{rpm_limit}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    label_class: "mt-4",
                                    name: "context_size",
                                    label: "Context Size",
                                    help_text: "How much data can be passed to the prompt",
                                    value: "{context_size_bytes}",
                                    required: true
                                }
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Capabilities",
                            div {
                                class: "flex flex-col mt-3",

                                if model_type == "LLM" {
                                    div {
                                        class: "form-control",
                                        label {
                                            class: "label cursor-pointer",
                                            span { class: "label-text", "Function Calling" }
                                            input {
                                                "type": "checkbox",
                                                name: "capability_function_calling",
                                                class: "checkbox",
                                                checked: has_capability_function_calling
                                            }
                                        }
                                    }

                                    div {
                                        class: "form-control",
                                        label {
                                            class: "label cursor-pointer",
                                            span { class: "label-text", "Vision" }
                                            input {
                                                "type": "checkbox",
                                                name: "capability_vision",
                                                class: "checkbox",
                                                checked: has_capability_vision
                                            }
                                        }
                                    }

                                    div {
                                        class: "form-control",
                                        label {
                                            class: "label cursor-pointer",
                                            span { class: "label-text", "Tool Use" }
                                            input {
                                                "type": "checkbox",
                                                name: "capability_tool_use",
                                                class: "checkbox",
                                                checked: has_capability_tool_use
                                            }
                                        }
                                    }
                                } else {
                                    p { "Capabilities are only available for LLM models." }
                                }
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Examples",
                            div {
                                class: "flex flex-col mt-3",

                                Input {
                                    input_type: InputType::Text,
                                    label: "Disclaimer",
                                    help_text: "This is displayed at the bottom of the chat.",
                                    name: "disclaimer",
                                    value: "{disclaimer}"
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label: "Example 1",
                                    label_class: "mt-4",
                                    help_text: "Give the user an example prompt.",
                                    name: "example1",
                                    value: "{example1}"
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label: "Example 2",
                                    label_class: "mt-4",
                                    help_text: "Give the user an example prompt.",
                                    name: "example2",
                                    value: "{example2}"
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label: "Example 3",
                                    label_class: "mt-4",
                                    help_text: "Give the user an example prompt.",
                                    name: "example3",
                                    value: "{example3}"
                                }

                                Input {
                                    input_type: InputType::Text,
                                    label: "Example 4",
                                    label_class: "mt-4",
                                    help_text: "Give the user an example prompt.",
                                    name: "example4",
                                    value: "{example4}"
                                }
                            }
                        }
                    }

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
