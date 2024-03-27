#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use dioxus::prelude::*;

#[component]
pub fn Form(
    cx: Scope,
    team_id: i32,
    name: String,
    base_url: String,
    model_type: String,
    trigger_id: String,
    api_key: String,
    billion_parameters: i32,
    context_size_bytes: i32,
    id: Option<i32>,
) -> Element {
    cx.render(rsx!(
        form {
            class: "form-control",
            action: "{crate::routes::models::new_route(*team_id)}",
            method: "post",
            Drawer {
                label: "Add a Model",
                trigger_id: "{*trigger_id}",
                DrawerBody {
                    div {
                        class: "flex flex-col",
                        if let Some(id) = id {
                            cx.render(rsx!(
                                input {
                                    "type": "hidden",
                                    value: "{id}",
                                    name: "id"
                                }
                            ))
                        }

                        Input {
                            input_type: InputType::Text,
                            label_class: "mt-4",
                            name: "name",
                            label: "Model Name",
                            help_text: "Make the name memorable and imply it's usage.",
                            value: &name,
                            required: true
                        }

                        Select {
                            name: "model_type",
                            label: "Is this model for LLM or Embeddings",
                            label_class: "mt-4",
                            help_text: "Some models can do both, in which case enter it twice.",
                            value: &model_type,
                            SelectOption {
                                value: "LLM",
                                selected_value: &model_type,
                                "Large Language Model"
                            }
                            SelectOption {
                                value: "Embeddings",
                                selected_value: &model_type,
                                "Embeddings Model"
                            }
                        }

                        Input {
                            input_type: InputType::Text,
                            label_class: "mt-4",
                            name: "base_url",
                            label: "The Base URL of the model",
                            help_text: "The URL location of the OpenAI compatible API",
                            value: &base_url,
                            required: true
                        }


                        Input {
                            input_type: InputType::Text,
                            label_class: "mt-4",
                            name: "api_key",
                            label: "The API secret from your provider",
                            help_text: "This will be given in the providers console",
                            value: &api_key
                        }

                        Input {
                            input_type: InputType::Number,
                            label_class: "mt-4",
                            name: "billion_parameters",
                            label: "How many billion parameters is the model",
                            help_text: "This is used only for information purposes.",
                            value: "{billion_parameters}",
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

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Save"
                    }
                }
            }
        }
    ))
}
