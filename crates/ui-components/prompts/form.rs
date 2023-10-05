#![allow(non_snake_case)]
use db::{Dataset, Model};
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct Props {
    trigger_id: String,
    organisation_id: i32,
    name: String,
    template: String,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    model_id: i32,
    id: Option<i32>,
    min_history_items: i32,
    max_history_items: i32,
    min_chunks: i32,
    max_chunks: i32,
    max_tokens: i32,
    temperature: f32,
    top_p: f32,
}

pub fn Form(cx: Scope<Props>) -> Element {
    cx.render(rsx!(
        form {
            action: "{crate::routes::models::new_route(cx.props.organisation_id)}",
            method: "post",
            Drawer {
                label: "Prompt",
                trigger_id: "{cx.props.trigger_id}",
                DrawerBody {
                    TabContainer {
                        tabs: cx.render(rsx! {
                            TabHeader {
                                selected: true,
                                tab: "all-panel",
                                name: "Prompt"
                            }
                            TabHeader {
                                selected: false,
                                tab: "datasets-panel",
                                name: "Datasets"
                            }
                            TabHeader {
                                selected: false,
                                tab: "advanced-panel",
                                name: "Advanced"
                            }
                        }),
                        TabPanel {
                            hidden: false,
                            id: "all-panel",
                            div {
                                class: "d-flex flex-column mt-3",
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
                                    label: "Prompt Name",
                                    help_text: "Make the name memorable and imply it's usage.",
                                    value: &cx.props.name,
                                    required: true
                                }

                                Select {
                                    name: "model_id",
                                    label: "Select the model this prompt will use for inference",
                                    help_text: "The prompt will be passed to the model",
                                    value: &cx.props.model_id.to_string(),
                                    required: true,
                                    cx.props.models.iter().map(|model| {
                                        cx.render(rsx!(
                                            option {
                                                value: "{model.id}",
                                                "{model.name}"
                                            }
                                        ))
                                    })

                                }

                                TextArea {
                                    class: "mt-3",
                                    name: "template",
                                    rows: "10",
                                    label: "Prompt Template",
                                    required: true,
                                    "{cx.props.template}",
                                }
                            }
                        }
                        TabPanel {
                            hidden: true,
                            id: "datasets-panel",
                            div {
                                class: "d-flex flex-column mt-3",
                                Select {
                                    name: "dataset_connection",
                                    label: "How shall we handle datasets with this prompt?",
                                    help_text: "The prompt will be passed to the model",
                                    value: &cx.props.name,
                                    required: true,
                                    option {
                                        value: "All",
                                        "Use All the Teams Datasets"
                                    }
                                    option {
                                        value: "None",
                                        "Don't use any datasets"
                                    }
                                    option {
                                        value: "Selected",
                                        "Use Selected Datasets"
                                    }
                                }

                                Select {
                                    name: "datasets",
                                    label: "Select datasets to connect to this prompt",
                                    help_text: "These datasets will only be used when the above is set to 'Use Selected Datasets'",
                                    value: &cx.props.name,
                                    multiple: true,
                                    cx.props.datasets.iter().map(|dataset| {
                                        cx.render(rsx!(
                                            option {
                                                value: "{dataset.id}",
                                                "{dataset.name}"
                                            }
                                        ))
                                    })
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "min_chunks",
                                    label: "Minimum number of Chunks",
                                    help_text: "As we retrieve text in batches whats the minimum we should add to the prompt",
                                    value: "{cx.props.min_chunks}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "max_chunks",
                                    label: "Maximum number of Chunks",
                                    help_text: "We don't add more chunks to the prompt than this.",
                                    value: "{cx.props.max_chunks}",
                                    required: true
                                }
                            }
                        }
                        TabPanel {
                            hidden: true,
                            id: "advanced-panel",
                            div {
                                class: "d-flex flex-column mt-3",

                                Input {
                                    input_type: InputType::Number,
                                    name: "temperature",
                                    label: "Temperature",
                                    help_text: "Value between 0 and 2.",
                                    value: "{cx.props.temperature}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "min_history_items",
                                    label: "Minimum number of history items",
                                    help_text: "This decides how much history we add to the prompt",
                                    value: "{cx.props.min_history_items}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "max_history_items",
                                    label: "Max number of history items",
                                    help_text: "This decides how much history we add to the prompt",
                                    value: "{cx.props.max_history_items}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "max_tokens",
                                    label: "Max Tokens",
                                    help_text: "How much of the context to leave for the LLM's reply",
                                    value: "{cx.props.max_tokens}",
                                    required: true
                                }

                                Input {
                                    input_type: InputType::Number,
                                    name: "top_p",
                                    label: "Alternative to Temperature",
                                    help_text: "Value between 0 and 2.",
                                    value: "{cx.props.top_p}",
                                    required: true
                                }
                            }
                        }
                    }

                }

                DrawerFooter {
                    Button {
                        button_type: ButtonType::Submit,
                        button_scheme: ButtonScheme::Primary,
                        "Submit"
                    }
                }
            }
        }
    ))
}
