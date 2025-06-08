#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Category, Dataset, Integration, Model, Visibility};
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct PromptForm {
    pub id: Option<i32>,
    pub selected_dataset_ids: Vec<i32>,
    pub selected_integration_ids: Vec<i32>,
    pub name: String,
    pub system_prompt: String,
    pub category_id: i32,
    pub model_id: i32,
    pub visibility: String,
    pub description: String,
    pub disclaimer: String,
    pub example1: Option<String>,
    pub example2: Option<String>,
    pub example3: Option<String>,
    pub example4: Option<String>,
    pub max_history_items: i32,
    pub max_chunks: i32,
    pub max_tokens: i32,
    pub trim_ratio: i32,
    pub temperature: f32,
    #[serde(skip)]
    pub error: Option<String>,
    #[serde(skip)]
    pub datasets: Vec<Dataset>,
    #[serde(skip)]
    pub integrations: Vec<Integration>,
    #[serde(skip)]
    pub categories: Vec<Category>,
    #[serde(skip)]
    pub models: Vec<Model>,
}

pub fn page(team_id: i32, rbac: Rbac, prompt: PromptForm) -> String {
    let example1 = prompt.example1.clone().unwrap_or_default();
    let example2 = prompt.example2.clone().unwrap_or_default();
    let example3 = prompt.example3.clone().unwrap_or_default();
    let example4 = prompt.example4.clone().unwrap_or_default();

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Assistant",
            header: rsx!(
                h3 {
                    if prompt.id.is_some() { "Edit Assistant" } else { "Create Assistant" }
                }
            ),

            Card {
                CardHeader {
                    title: if prompt.id.is_some() { "Edit Assistant" } else { "Create Assistant" }
                }
                CardBody {
                    form {
                        action: crate::routes::prompts::Upsert { team_id }.to_string(),
                        enctype: "multipart/form-data",
                        method: "post",
                        class: "flex flex-col space-y-6",

                        // Display error if present
                        if let Some(error) = &prompt.error {
                            div {
                                class: "alert alert-error mb-4",
                                "{error}"
                            }
                        }

                        // Hidden ID field for edit mode
                        if let Some(id) = prompt.id {
                            input {
                                "type": "hidden",
                                value: "{id}",
                                name: "id"
                            }
                        }

                        // Assistant Details Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Assistant Details" }

                            Input {
                                input_type: InputType::Text,
                                name: "name",
                                label: "Assistant Name",
                                help_text: "Make the name memorable and imply its usage.",
                                value: prompt.name,
                                required: true
                            }

                            Select {
                                name: "category_id",
                                label: "Select the category for this assistant",
                                label_class: "mt-4",
                                help_text: "Categories help users find assistants.",
                                value: "{prompt.category_id}",
                                required: true,
                                for category in &prompt.categories {
                                    SelectOption {
                                        value: "{category.id}",
                                        selected_value: "{prompt.category_id}",
                                        "{category.name}"
                                    }
                                }
                            }

                            Select {
                                name: "visibility",
                                label: "Who should be able to use this assistant?",
                                label_class: "mt-4",
                                help_text: "Set to private if you don't want to share this assistant.",
                                value: "{prompt.visibility}",
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Private)}",
                                    selected_value: "{prompt.visibility}",
                                    {crate::visibility_to_string(Visibility::Private)}
                                },
                                SelectOption {
                                    value: "{crate::visibility_to_string(Visibility::Team)}",
                                    selected_value: "{prompt.visibility}",
                                    {crate::visibility_to_string(Visibility::Team)}
                                },
                                if rbac.can_make_assistant_public() {
                                    SelectOption {
                                        value: "{crate::visibility_to_string(Visibility::Company)}",
                                        selected_value: "{prompt.visibility}",
                                        {crate::visibility_to_string(Visibility::Company)}
                                    }
                                }
                            }

                            Select {
                                name: "model_id",
                                label: "Select the model this assistant will use",
                                label_class: "mt-4",
                                help_text: "The model will be used to answer any questions.",
                                value: "{prompt.model_id}",
                                required: true,
                                for model in &prompt.models {
                                    SelectOption {
                                        value: "{model.id}",
                                        selected_value: "{prompt.model_id}",
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
                                "{prompt.description}",
                            }
                        }

                        // System Prompt Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "System Prompt" }

                            TextArea {
                                class: "mt-3 font-mono leading-tight overflow-y-auto",
                                name: "system_prompt",
                                rows: "16",
                                label: "System Prompt",
                                label_class: "mt-4",
                                "{prompt.system_prompt}",
                            }
                        }

                        // Assistant Icon Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Assistant Icon" }

                            div {
                                class: "mt-3",
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Upload Assistant Icon"
                                }
                                input {
                                    "type": "file",
                                    name: "image_icon",
                                    accept: "image/*",
                                    class: "block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
                                }
                            }
                        }

                        // Datasets Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Datasets" }

                            Alert {
                                class: "mb-4",
                                "Select which datasets you wish to attach to this assistant"
                            }

                            if !prompt.datasets.is_empty() {
                                table {
                                    class: "table table-sm w-full",
                                    thead {
                                        tr {
                                            th { "Dataset" }
                                            th { "Model" }
                                            th { "Add?" }
                                        }
                                    }
                                    tbody {
                                        for dataset in &prompt.datasets {
                                            tr {
                                                td { "{dataset.name}" }
                                                td { "{dataset.embeddings_model_name}" }
                                                td {
                                                    if prompt.selected_dataset_ids.contains(&dataset.id) {
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
                            } else {
                                div {
                                    class: "text-gray-500 italic",
                                    "No datasets available"
                                }
                            }
                        }

                        // Integrations Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Integrations" }

                            Alert {
                                class: "mb-4",
                                "Select which integrations this assistant can use"
                            }

                            if !prompt.integrations.is_empty() {
                                table {
                                    class: "table table-sm w-full",
                                    thead {
                                        tr {
                                            th { "Integration" }
                                            th { "Type" }
                                            th { "Status" }
                                            th { "Add?" }
                                        }
                                    }
                                    tbody {
                                        for integration in &prompt.integrations {
                                            tr {
                                                td { "{integration.name}" }
                                                td { "{integration.integration_type:?}" }
                                                td {
                                                    span {
                                                        class: match integration.integration_status {
                                                            db::IntegrationStatus::Configured => "badge badge-success",
                                                            _ => "badge badge-warning"
                                                        },
                                                        "{integration.integration_status:?}"
                                                    }
                                                }
                                                td {
                                                    if prompt.selected_integration_ids.contains(&integration.id) {
                                                        CheckBox {
                                                            checked: true,
                                                            name: "integrations",
                                                            value: "{integration.id}"
                                                        }
                                                    } else {
                                                        CheckBox {
                                                            name: "integrations",
                                                            value: "{integration.id}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                div {
                                    class: "text-gray-500 italic",
                                    "No integrations available"
                                }
                            }
                        }

                        // Examples Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Examples" }

                            Input {
                                input_type: InputType::Text,
                                label: "Disclaimer",
                                help_text: "This is displayed at the bottom of the chat.",
                                name: "disclaimer",
                                value: "{prompt.disclaimer}"
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

                        // Advanced Settings Section
                        div {
                            class: "flex flex-col space-y-4",
                            h4 { class: "text-lg font-semibold text-gray-900", "Advanced Settings" }

                            Input {
                                input_type: InputType::Number,
                                step: "0.1",
                                name: "temperature",
                                label: "Temperature",
                                help_text: "Value between 0 and 2.",
                                value: "{prompt.temperature}",
                                required: true
                            }

                            Input {
                                input_type: InputType::Number,
                                name: "max_history_items",
                                label: "Max number of history items",
                                label_class: "mt-4",
                                help_text: "This decides how much history we add to the prompt. Set it to zero to send no history.",
                                value: "{prompt.max_history_items}",
                                required: true
                            }

                            Input {
                                input_type: InputType::Number,
                                name: "max_tokens",
                                label: "Max Tokens",
                                label_class: "mt-4",
                                help_text: "How much of the context to leave for the LLM's reply. Set this to roughly half of the available context for the model you are using.",
                                value: "{prompt.max_tokens}",
                                required: true
                            }

                            Range {
                                label: "Trim Ratio",
                                label_class: "mt-4",
                                name: "trim_ratio",
                                min: 0,
                                max: 100,
                                value: prompt.trim_ratio,
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
                                value: "{prompt.max_chunks}",
                                required: true
                            }
                        }

                        // Form Actions
                        div {
                            class: "mt-8 flex justify-between",
                            Button {
                                button_type: ButtonType::Link,
                                href: crate::routes::prompts::Index { team_id }.to_string(),
                                button_scheme: ButtonScheme::Error,
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
        }
    };

    crate::render(page)
}
