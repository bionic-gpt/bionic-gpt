#![allow(non_snake_case)]
use daisy_rsx::*;
use db::{Category, Dataset, Model, Visibility};
use dioxus::prelude::*;

// Main Form component using the separated tabs
#[component]
pub fn Form(
    trigger_id: String,
    team_id: i32,
    name: String,
    system_prompt: String,
    datasets: Vec<Dataset>,
    selected_dataset_ids: Vec<i32>,
    models: Vec<Model>,
    categories: Vec<Category>,
    category_id: i32,
    model_id: i32,
    visibility: Visibility,
    id: Option<i32>,
    max_history_items: i32,
    max_chunks: i32,
    max_tokens: i32,
    trim_ratio: i32,
    temperature: f32,
    description: String,
    disclaimer: String,
    example1: Option<String>,
    example2: Option<String>,
    example3: Option<String>,
    example4: Option<String>,
    can_make_assistant_public: bool,
) -> Element {
    let example1 = example1.unwrap_or("".to_string());
    let example2 = example2.unwrap_or("".to_string());
    let example3 = example3.unwrap_or("".to_string());
    let example4 = example4.unwrap_or("".to_string());
    rsx!(
        form {
            action: crate::routes::prompts::Upsert { team_id }.to_string(),
            enctype: "multipart/form-data",
            method: "post",
            Modal {
                trigger_id: "{trigger_id}",
                ModalBody {
                    class: "flex flex-col justify-between md:w-full max-w-[64rem] h-full",
                    TabContainer {
                        TabPanel {
                            checked: true,
                            name: "prompt-tabs",
                            tab_name: "Assistant",
                            AssistantTab {
                                name: name.clone(),
                                id,
                                categories: categories.clone(),
                                category_id,
                                visibility,
                                models: models.clone(),
                                model_id,
                                description: description.clone(),
                                can_make_assistant_public
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "System Prompt",
                            SystemPromptTab {
                                system_prompt: system_prompt.clone()
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Assistant Icon",
                            AssistantIconTab {}
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Datasets",
                            DatasetsTab {
                                datasets: datasets.clone(),
                                selected_dataset_ids: selected_dataset_ids.clone()
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Examples",
                            ExamplesTab {
                                disclaimer: disclaimer.clone(),
                                example1,
                                example2,
                                example3,
                                example4
                            }
                        }
                        TabPanel {
                            name: "prompt-tabs",
                            tab_name: "Advanced",
                            AdvancedTab {
                                temperature,
                                max_history_items,
                                max_tokens,
                                trim_ratio,
                                max_chunks
                            }
                        }
                    }

                    ModalAction {
                        Button {
                            class: "cancel-modal",
                            button_type: ButtonType::Reset,
                            button_scheme: ButtonScheme::Danger,
                            "Cancel"
                        }
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "Submit"
                        }
                    }
                }
            }
        }
    )
}

// Define the Assistant tab
#[component]
fn AssistantTab(
    name: String,
    id: Option<i32>,
    categories: Vec<Category>,
    category_id: i32,
    visibility: Visibility,
    models: Vec<Model>,
    model_id: i32,
    description: String,
    can_make_assistant_public: bool,
) -> Element {
    rsx!(
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
                help_text: "Make the name memorable and imply its usage.",
                value: name,
                required: true
            }

            Select {
                name: "category_id",
                label: "Select the category for this assistant",
                label_class: "mt-4",
                help_text: "Categories help users find assistants.",
                value: "{category_id}",
                required: true,
                for category in categories {
                    SelectOption {
                        value: "{category.id}",
                        selected_value: "{category_id}",
                        "{category.name}"
                    }
                }
            }

            Select {
                name: "visibility",
                label: "Who should be able to use this assistant?",
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
                if can_make_assistant_public {
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
                class: "mt-3 resize-none",
                name: "description",
                rows: "2",
                label: "Description",
                label_class: "mt-4",
                required: true,
                "{description}",
            }
        }
    )
}

// Define the System Prompt tab
#[component]
fn SystemPromptTab(system_prompt: String) -> Element {
    rsx!(
        div {
            class: "flex flex-col mt-3",
            TextArea {
                class: "mt-3 font-mono leading-tight overflow-y-auto",
                name: "system_prompt",
                rows: "32",
                label: "System Prompt",
                label_class: "mt-4",
                "{system_prompt}",
            }
        }
    )
}

// Define the Assistant Icon tab
#[component]
fn AssistantIconTab() -> Element {
    rsx!(
        div {
            class: "flex flex-col mt-3",
            input {
                "type": "file",
                name: "image_icon",
                accept: "image/*"
            }
        }
    )
}

// Define the Datasets tab
#[component]
fn DatasetsTab(datasets: Vec<Dataset>, selected_dataset_ids: Vec<i32>) -> Element {
    rsx!(
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
                        th { "Dataset" }
                        th { "Model" }
                        th { "Add?" }
                    }
                }
                tbody {
                    for dataset in datasets {
                        tr {
                            td { "{dataset.name}" }
                            td { "{dataset.embeddings_model_name}" }
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
    )
}

// Define the Examples tab
#[component]
fn ExamplesTab(
    disclaimer: String,
    example1: String,
    example2: String,
    example3: String,
    example4: String,
) -> Element {
    rsx!(
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
    )
}

// Define the Advanced tab
#[component]
fn AdvancedTab(
    temperature: f32,
    max_history_items: i32,
    max_tokens: i32,
    trim_ratio: i32,
    max_chunks: i32,
) -> Element {
    rsx!(
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
                help_text: "This decides how much history we add to the prompt. Set it to zero to send no history.",
                value: "{max_history_items}",
                required: true
            }

            Input {
                input_type: InputType::Number,
                name: "max_tokens",
                label: "Max Tokens",
                label_class: "mt-4",
                help_text: "How much of the context to leave for the LLM's reply. Set this to roughly half of the available context for the model you are using.",
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
                help_text: "The way we count tokens may not match the way the inference engine does. Here you can say how much of the available context to use. i.e. 80% will use 80% of the context_size - max_tokens.",
                div {
                    class: "w-full flex justify-between text-xs px-2",
                    span { "0" }
                    span { "20" }
                    span { "40" }
                    span { "60" }
                    span { "80" }
                    span { "100" }
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
    )
}
