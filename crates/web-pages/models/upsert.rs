#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::{select::SelectOption, *};
use db::authz::Rbac;
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug, Clone)]
pub struct ModelForm {
    pub id: Option<i32>,
    pub prompt_id: Option<i32>,
    pub name: String,
    pub display_name: String,
    pub model_type: String,
    pub base_url: String,
    pub api_key: String,
    pub tpm_limit: i32,
    pub rpm_limit: i32,
    pub context_size_bytes: i32,
    pub disclaimer: String,
    pub description: String,
    pub example1: String,
    pub example2: String,
    pub example3: String,
    pub example4: String,
    pub has_capability_function_calling: bool,
    pub has_capability_vision: bool,
    pub has_capability_tool_use: bool,
    #[serde(skip)]
    pub error: Option<String>,
}

pub fn page(team_id: i32, rbac: Rbac, form: ModelForm) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Models,
            team_id: team_id,
            rbac: rbac,
            title: "Models",
            header: rsx!(
                h3 { if form.id.is_some() { "Edit Model" } else { "Create Model" } }
            ),
            div {
                class: "p-4 max-w-4xl w-full mx-auto",
                form {
                    action: crate::routes::models::Upsert { team_id }.to_string(),
                    method: "post",
                    class: "space-y-6",
                    if let Some(id) = form.id {
                        input {
                            "type": "hidden",
                            value: "{id}",
                            name: "id"
                        }
                    }
                    if let Some(pid) = form.prompt_id {
                        input {
                            "type": "hidden",
                            value: "{pid}",
                            name: "prompt_id"
                        }
                    }
                    // Model Details
                    Card {
                        class: "mb-6",
                        CardHeader { title: "Model Details" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        name: "display_name",
                                        label: "Display Name",
                                        label_class: "mt-4",
                                        help_text: "Make the name memorable and imply it's usage.",
                                        value: form.display_name.clone(),
                                        required: true
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label_class: "mt-4",
                                        name: "name",
                                        label: "Model Name",
                                        help_text: "The model's id as used in the API. i.e. llama3-70b",
                                        value: form.name.clone(),
                                        required: true
                                    }
                                }
                            }
                            div {
                                    class: "flex flex-col",
                                TextArea {
                                    class: "mt-3 w-full",
                                    name: "description",
                                    rows: "8",
                                    label: "Description",
                                    help_text: "A brief summary about this model.",
                                    label_class: "mt-4",
                                    required: true,
                                    "{form.description}"
                                }
                            }
                            div {
                                class: "flex flex-col",
                                Select {
                                    name: "model_type",
                                    label: "Is this model for LLM or Embeddings",
                                    label_class: "mt-4",
                                    help_text: "Some models can do both, in which case enter it twice.",
                                    value: form.model_type.clone(),
                                    SelectOption { value: "LLM", selected_value: form.model_type.clone(), "Large Language Model" }
                                    SelectOption { value: "Embeddings", selected_value: form.model_type.clone(), "Embeddings Model" }
                                    SelectOption { value: "Image", selected_value: form.model_type.clone(), "Image Generation" }
                                    SelectOption { value: "TextToSpeech", selected_value: form.model_type.clone(), "Text To Speech" }
                                }
                            }
                            div {
                                class: "flex flex-col",
                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "base_url",
                                    label: "The Base URL of the model",
                                    help_text: "The URL location of the OpenAI compatible API",
                                    value: form.base_url.clone(),
                                    required: true
                                }
                            }
                            div {
                                class: "flex flex-col",
                                Input {
                                    input_type: InputType::Text,
                                    label_class: "mt-4",
                                    name: "api_key",
                                    label: "The API secret from your provider",
                                    help_text: "This will be given in the providers console",
                                    value: form.api_key.clone()
                                }
                            }
                        }
                    }

                    // Advanced Settings
                    Card {
                        class: "mb-6",
                        CardHeader { title: "Advanced Settings" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            Input {
                                input_type: InputType::Number,
                                label_class: "mt-4",
                                name: "tpm_limit",
                                label: "Set the maximum tokens per minute for each user.",
                                help_text: "If users exceed this limit there access to the model will be limited.",
                                value: "{form.tpm_limit}",
                                required: true
                            }
                            Input {
                                input_type: InputType::Number,
                                label_class: "mt-4",
                                name: "rpm_limit",
                                label: "Set the maximum requests per minute for each user.",
                                help_text: "If users exceed this limit there access to the model will be limited.",
                                value: "{form.rpm_limit}",
                                required: true
                            }
                            Input {
                                input_type: InputType::Number,
                                label_class: "mt-4",
                                name: "context_size",
                                label: "Context Size",
                                help_text: "How much data can be passed to the prompt",
                                value: "{form.context_size_bytes}",
                                required: true
                            }
                        }
                    }

                    // Capabilities
                    Card {
                        class: "mb-6",
                        CardHeader { title: "Capabilities" }
                        CardBody {
                            class: "flex flex-col gap-4",
                            if form.model_type == "LLM" {
                                div {
                                    class: "form-control",
                                    label {
                                        class: "label cursor-pointer",
                                        span { class: "label-text", "Function Calling" }
                                        input { "type": "checkbox", name: "capability_function_calling", class: "checkbox", checked: form.has_capability_function_calling }
                                    }
                                }
                                div {
                                    class: "form-control",
                                    label {
                                        class: "label cursor-pointer",
                                        span { class: "label-text", "Vision" }
                                        input { "type": "checkbox", name: "capability_vision", class: "checkbox", checked: form.has_capability_vision }
                                    }
                                }
                                div {
                                    class: "form-control",
                                    label {
                                        class: "label cursor-pointer",
                                        span { class: "label-text", "Tool Use" }
                                        input { "type": "checkbox", name: "capability_tool_use", class: "checkbox", checked: form.has_capability_tool_use }
                                    }
                                }
                            } else {
                                p { "Capabilities are only available for LLM models." }
                            }
                        }
                    }

                    // Examples & Disclaimer
                    Card {
                        class: "mb-6",
                        CardHeader { title: "Examples & Disclaimer" }
                        CardBody {
                            class: "flex flex-col gap-6",
                            Input {
                                input_type: InputType::Text,
                                label: "Disclaimer",
                                help_text: "This is displayed at the bottom of the chat.",
                                name: "disclaimer",
                                value: "{form.disclaimer}"
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 1",
                                        help_text: "Give the user an example prompt.",
                                        name: "example1",
                                        value: "{form.example1}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 2",
                                        help_text: "Give the user an example prompt.",
                                        name: "example2",
                                        value: "{form.example2}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 3",
                                        help_text: "Give the user an example prompt.",
                                        name: "example3",
                                        value: "{form.example3}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 4",
                                        help_text: "Give the user an example prompt.",
                                        name: "example4",
                                        value: "{form.example4}"
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex justify-between mt-4",
                        Button {
                            button_type: ButtonType::Link,
                            href: crate::routes::models::Index { team_id }.to_string(),
                            button_scheme: ButtonScheme::Error,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            if form.id.is_some() { "Update Model" } else { "Create Model" }
                        }
                    }
                }
            }
        }
    };
    crate::render(page)
}
