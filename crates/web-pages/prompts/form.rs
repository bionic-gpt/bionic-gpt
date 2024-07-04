#![allow(non_snake_case)]
use daisy_rsx::*;
use db::{Dataset, Model, Visibility};
use dioxus::prelude::*;

#[component]
pub fn Form(
    trigger_id: String,
    team_id: i32,
    name: String,
    system_prompt: String,
    datasets: Vec<Dataset>,
    selected_dataset_ids: Vec<i32>,
    models: Vec<Model>,
    model_id: i32,
    visibility: Visibility,
    id: Option<i32>,
    max_history_items: i32,
    max_chunks: i32,
    max_tokens: i32,
    trim_ratio: i32,
    temperature: f32,
    is_saas: bool,
) -> Element {
    rsx!(
        Drawer {
            label: "Assistant",
            submit_action: crate::routes::prompts::Upsert{team_id}.to_string(),
            trigger_id: "{trigger_id}",
            DrawerBody {
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

                            Input {
                                input_type: InputType::Text,
                                name: "name",
                                label: "Assistant Name",
                                help_text: "Make the name memorable and imply it's usage.",
                                value: name,
                                required: true
                            }

                            Select {
                                name: "visibility",
                                label: "Who should be able to usee this assistant?",
                                label_class: "mt-4",
                                help_text: "Set to private if you don't want to share this assistant.",
                                value: "{crate::visibility_to_string(visibility)}",
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Private)}",
                                    selected_value: "{crate::visibility_to_string(visibility)}",
                                    {crate::visibility_to_string(Visibility::Private)}
                                },
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Team)}",
                                    selected_value: "{crate::visibility_to_string(visibility)}",
                                    {crate::visibility_to_string(Visibility::Team)}
                                },
                                if ! is_saas {
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Company)}",
                                        selected_value: "{crate::visibility_to_string(visibility)}",
                                        {crate::visibility_to_string(Visibility::Company)}
                                    }
                                }
                            }

                            Select {
                                name: "model_id",
                                label: "Select the model this assistant will use",
                                label_class: "mt-4",
                                help_text: "The model will be used to answer any questions.",
                                value: "{model_id}",
                                required: true,
                                for model in models {
                                    SelectOption {
                                        value: "{model.id}",
                                        selected_value: "{model_id}",
                                        "{model.name}"
                                    }
                                }
                            }

                            TextArea {
                                class: "mt-3",
                                name: "system_prompt",
                                rows: "10",
                                label: "System Prompt",
                                label_class: "mt-4",
                                "{system_prompt}",
                            }
                        }
                    }
                    TabPanel {
                        name: "prompt-tabs",
                        tab_name: "Datasets",
                        div {
                            class: "flex flex-col mt-3",
                            Alert {
                                class: "mb-4",
                                "Select which datasets you wish to attach to this assistant"
                            }
                            table {
                                class: "table table-sm",
                                thead {
                                    tr {
                                        th {
                                            "Dataset"
                                        }
                                        th {
                                            "Model"
                                        }
                                        th {
                                            "Add?"
                                        }
                                    }
                                }
                                tbody {
                                    for dataset in datasets {
                                        tr {
                                            td {
                                                "{dataset.name}"
                                            }
                                            td {
                                                "{dataset.embeddings_model_name}"
                                            }
                                            td {
                                                if selected_dataset_ids.contains(&dataset.id) {
                                                    CheckBox {
                                                        checked: true,
                                                        name: "datasets",
                                                        value: "{dataset.id}"
                                                    }
                                                } else {
                                                    CheckBox {
                                                        name: "datasets",
                                                        value: "{dataset.id}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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
                                step: "0.1",
                                name: "temperature",
                                label: "Temperature",
                                help_text: "Value between 0 and 2.",
                                value: "{temperature}",
                                required: true
                            }

                            Input {
                                input_type: InputType::Number,
                                name: "max_history_items",
                                label: "Max number of history items",
                                label_class: "mt-4",
                                help_text: "This decides how much history we add to the prompt.
                                Set it to zero to send no history.",
                                value: "{max_history_items}",
                                required: true
                            }

                            Input {
                                input_type: InputType::Number,
                                name: "max_tokens",
                                label: "Max Tokens",
                                label_class: "mt-4",
                                help_text: "How much of the context to leave for the LLM's reply.
                                Set this to roughly half of the available context for the model you are using.",
                                value: "{max_tokens}",
                                required: true
                            }

                            Range {
                                label: "Trim Ratio",
                                label_class: "mt-4",
                                name: "trim_ratio",
                                min: 0,
                                max: 100,
                                value: trim_ratio,
                                help_text: "The way we count tokens may not match the way the the inference engine does.
                                Here you can say how much of the available context to use. i.e. 80% will use 80% of the context_size - max_tokens.",
                                div {
                                    class: "w-full flex justify-between text-xs px-2",
                                    span {
                                        "0"
                                    }
                                    span {
                                        "20"
                                    }
                                    span {
                                        "40"
                                    }
                                    span {
                                        "60"
                                    }
                                    span {
                                        "80"
                                    }
                                    span {
                                        "100"
                                    }
                                }
                            }

                            Input {
                                input_type: InputType::Number,
                                name: "max_chunks",
                                label: "Maximum number of Chunks",
                                label_class: "mt-4",
                                help_text: "We don't add more chunks to the prompt than this.",
                                value: "{max_chunks}",
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
    )
}
