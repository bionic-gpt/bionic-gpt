#![allow(non_snake_case)]
use super::dataset_connection_to_string;
use db::{Dataset, DatasetConnection, Model, Visibility};
use dioxus::prelude::*;
use primer_rsx::{select::SelectOption, *};

#[derive(Props, PartialEq)]
pub struct Props {
    trigger_id: String,
    organisation_id: i32,
    name: String,
    system_prompt: String,
    datasets: Vec<Dataset>,
    selected_dataset_ids: Vec<i32>,
    dataset_connection: DatasetConnection,
    models: Vec<Model>,
    model_id: i32,
    visibility: Visibility,
    id: Option<i32>,
    max_history_items: i32,
    max_chunks: i32,
    max_tokens: i32,
    temperature: f32,
    top_p: f32,
}

pub fn Form(cx: Scope<Props>) -> Element {
    cx.render(rsx!(
        form {
            action: "{crate::routes::prompts::new_route(cx.props.organisation_id)}",
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
                                class: "flex flex-col mt-3",
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
                                    name: "visibility",
                                    label: "Who should be able to see this prompt?",
                                    help_text: "Set to private if you don't want to share this prompt.",
                                    value: "{crate::visibility_to_string(cx.props.visibility)}",
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Private)}",
                                        selected_value: "{crate::visibility_to_string(cx.props.visibility)}",
                                        crate::visibility_to_string(Visibility::Private)
                                    },
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Team)}",
                                        selected_value: "{crate::visibility_to_string(cx.props.visibility)}",
                                        crate::visibility_to_string(Visibility::Team)
                                    },
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Company)}",
                                        selected_value: "{crate::visibility_to_string(cx.props.visibility)}",
                                        crate::visibility_to_string(Visibility::Company)
                                    }
                                }

                                Select {
                                    name: "model_id",
                                    label: "Select the model this prompt will use for inference",
                                    help_text: "The prompt will be passed to the model",
                                    value: &cx.props.model_id.to_string(),
                                    required: true,
                                    cx.props.models.iter().map(|model| {
                                        cx.render(rsx!(
                                            SelectOption {
                                                value: "{model.id}",
                                                selected_value: "{cx.props.model_id}",
                                                "{model.name}"
                                            }
                                        ))
                                    })

                                }

                                TextArea {
                                    class: "mt-3",
                                    name: "system_prompt",
                                    rows: "10",
                                    label: "Prompt",
                                    "{cx.props.system_prompt}",
                                }
                            }
                        }
                        TabPanel {
                            hidden: true,
                            id: "datasets-panel",
                            div {
                                class: "flex flex-col mt-3",
                                Select {
                                    name: "dataset_connection",
                                    label: "How shall we handle datasets with this prompt?",
                                    help_text: "The prompt will be passed to the model",
                                    value: "{dataset_connection_to_string(cx.props.dataset_connection)}",
                                    required: true,
                                    SelectOption {
                                        value: "{dataset_connection_to_string(DatasetConnection::All)}",
                                        selected_value: "{dataset_connection_to_string(cx.props.dataset_connection)}",
                                        dataset_connection_to_string(DatasetConnection::All)
                                    }
                                    SelectOption {
                                        value: "{dataset_connection_to_string(DatasetConnection::None)}",
                                        selected_value: "{dataset_connection_to_string(cx.props.dataset_connection)}",
                                        dataset_connection_to_string(DatasetConnection::None)
                                    }
                                    SelectOption {
                                        value: "{dataset_connection_to_string(DatasetConnection::Selected)}",
                                        selected_value: "{dataset_connection_to_string(cx.props.dataset_connection)}",
                                        dataset_connection_to_string(DatasetConnection::Selected)
                                    }
                                }

                                Select {
                                    name: "datasets",
                                    label: "Select datasets to connect to this prompt",
                                    help_text: "These datasets will only be used when the above is set to 'Use Selected Datasets'",
                                    value: &cx.props.name,
                                    multiple: true,
                                    cx.props.datasets.iter().map(|dataset| {
                                        if cx.props.selected_dataset_ids.contains(&dataset.id) {
                                            cx.render(rsx!(
                                                option {
                                                    value: "{dataset.id}",
                                                    selected: true,
                                                    "{dataset.name}"
                                                }
                                            ))
                                        } else {
                                            cx.render(rsx!(
                                                option {
                                                    value: "{dataset.id}",
                                                    "{dataset.name}"
                                                }
                                            ))
                                        }
                                    })
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
                                class: "flex flex-col mt-3",

                                Input {
                                    input_type: InputType::Number,
                                    step: "0.1",
                                    name: "temperature",
                                    label: "Temperature",
                                    help_text: "Value between 0 and 2.",
                                    value: "{cx.props.temperature}",
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
