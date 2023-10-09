#![allow(non_snake_case)]
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    organisation_id: i32,
    name: String,
    base_url: String,
    model_type: String,
    trigger_id: String,
    api_key: String,
    billion_parameters: i32,
    context_size_bytes: i32,
    id: Option<i32>,
}

pub fn Form(cx: Scope<Props>) -> Element {
    cx.render(rsx!(
        form {
            action: "{crate::routes::models::new_route(cx.props.organisation_id)}",
            method: "post",
            Drawer {
                label: "Add a Model",
                trigger_id: "{cx.props.trigger_id}",
                DrawerBody {
                    div {
                        class: "d-flex flex-column",
                        if let Some(id) = cx.props.id {
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
                            name: "name",
                            label: "Model Name",
                            help_text: "Make the name memorable and imply it's usage.",
                            value: &cx.props.name,
                            required: true
                        }

                        Select {
                            name: "model_type",
                            label: "Is this model for LLM or Embeddings",
                            help_text: "Some models can do both, in which case enter it twice.",
                            value: &cx.props.model_type,
                            SelectOption {
                                value: "LLM",
                                selected_value: &cx.props.model_type,
                                "Large Language Model"
                            }
                            SelectOption {
                                value: "Embeddings",
                                selected_value: &cx.props.model_type,
                                "Embeddings Model"
                            }
                        }

                        Input {
                            input_type: InputType::Text,
                            name: "base_url",
                            label: "The Base URL of the model",
                            help_text: "The URL location of the OpenAI compatible API",
                            value: &cx.props.base_url,
                            required: true
                        }


                        Input {
                            input_type: InputType::Text,
                            name: "api_key",
                            label: "The API secret from your provider",
                            help_text: "This will be given in the providers console",
                            value: &cx.props.api_key
                        }

                        Input {
                            input_type: InputType::Number,
                            name: "billion_parameters",
                            label: "How many billion parameters is the model",
                            help_text: "This is used only for information purposes.",
                            value: "{cx.props.billion_parameters}",
                            required: true
                        }

                        Input {
                            input_type: InputType::Number,
                            name: "context_size",
                            label: "Context Size",
                            help_text: "How much data can be passed to the prompt",
                            value: "{cx.props.context_size_bytes}",
                            required: true
                        }
                    }
                }

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Add Model"
                    }
                }
            }
        }
    ))
}
