#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Category, Model, Visibility};
use dioxus::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug, Clone, PartialEq)]
pub struct PromptForm {
    pub id: Option<i32>,
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
    pub categories: Vec<Category>,
    #[serde(skip)]
    pub models: Vec<Model>,
}

pub fn page(team_id: i32, rbac: Rbac, prompt: PromptForm) -> String {
    let example1 = prompt.example1.clone().unwrap_or_default();
    let example2 = prompt.example2.clone().unwrap_or_default();
    let example3 = prompt.example3.clone().unwrap_or_default();
    let example4 = prompt.example4.clone().unwrap_or_default();
    let name = if prompt.id.is_some() {
        "Edit Assistant"
    } else {
        "Create Assistant"
    };

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Assistant",
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: "Assistants".into(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: "My Assistants".into(),
                            href: Some(crate::routes::prompts::MyAssistants{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: name.into(),
                            href: None
                        }
                    ]
                }
                h3 {
                    if prompt.id.is_some() { "Edit Assistant" } else { "Create Assistant" }
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",

                form {
                    action: crate::routes::prompts::Upsert { team_id }.to_string(),
                    enctype: "multipart/form-data",
                    method: "post",
                    class: "space-y-6",

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
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Assistant Details"
                        }
                        CardBody {
                            div {
                                class: "flex flex-col gap-6",

                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "name",
                                            label: "Assistant Name",
                                            help_text: "Make the name memorable and imply its usage.",
                                            value: prompt.name,
                                            required: true
                                        }
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Select {
                                            name: "category_id",
                                            label: "Category",
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
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Select {
                                            name: "visibility",
                                            label: "Visibility",
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
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Select {
                                            name: "model_id",
                                            label: "Model",
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
                                    }
                                }

                                div {
                                    class: "flex flex-col",
                                    TextArea {
                                        class: "resize-none w-full",
                                        name: "description",
                                        rows: "3",
                                        label: "Description",
                                        help_text: "Describe what this assistant does and how it can help users.",
                                        required: true,
                                        "{prompt.description}",
                                    }
                                }
                            }
                        }
                    }

                    // System Prompt Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Instructions"
                        }
                        CardBody {
                            class: "flex flex-col",
                            TextArea {
                                class: "font-mono leading-tight overflow-y-auto w-full",
                                name: "system_prompt",
                                rows: "16",
                                help_text: "Define the assistant's behavior, personality, and capabilities.",
                                "{prompt.system_prompt}",
                            }
                        }
                    }

                    // Assistant Icon Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Assistant Icon"
                        }
                        CardBody {
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-2",
                                    "Upload Assistant Icon"
                                }
                                p {
                                    class: "text-sm text-gray-500 mb-3",
                                    "Choose an image that represents your assistant. Recommended size: 48x48 pixels."
                                }
                                input {
                                    "type": "file",
                                    name: "image_icon",
                                    accept: "image/*",
                                    class: "block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
                                }
                            }
                        }
                    }

                    // Examples Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Examples & Disclaimer"
                        }
                        CardBody {
                            class: "flex flex-col gap-6",
                            div {
                                class: "flex flex-col",
                                Input {
                                    input_type: InputType::Text,
                                    label: "Disclaimer",
                                    help_text: "This is displayed at the bottom of the chat.",
                                    name: "disclaimer",
                                    value: "{prompt.disclaimer}"
                                }
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
                                        value: "{example1}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 2",
                                        help_text: "Give the user an example prompt.",
                                        name: "example2",
                                        value: "{example2}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 3",
                                        help_text: "Give the user an example prompt.",
                                        name: "example3",
                                        value: "{example3}"
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Input {
                                        input_type: InputType::Text,
                                        label: "Example 4",
                                        help_text: "Give the user an example prompt.",
                                        name: "example4",
                                        value: "{example4}"
                                    }
                                }
                            }
                        }
                    }

                    // Advanced Settings Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: "Advanced Settings"
                        }
                        CardBody {
                            div {
                                class: "space-y-4",

                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",

                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            input_type: InputType::Number,
                                            step: "0.1",
                                            name: "temperature",
                                            label: "Temperature",
                                            help_text: "Value between 0 and 2. Higher values make output more random.",
                                            value: "{prompt.temperature}",
                                            required: true
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            input_type: InputType::Number,
                                            name: "max_history_items",
                                            label: "Max History Items",
                                            help_text: "How much conversation history to include. Set to zero for no history.",
                                            value: "{prompt.max_history_items}",
                                            required: true
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            input_type: InputType::Number,
                                            name: "max_tokens",
                                            label: "Max Tokens",
                                            help_text: "Context space reserved for the LLM's reply.",
                                            value: "{prompt.max_tokens}",
                                            required: true
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            input_type: InputType::Number,
                                            name: "max_chunks",
                                            label: "Max Chunks",
                                            help_text: "Maximum number of dataset chunks to include.",
                                            value: "{prompt.max_chunks}",
                                            required: true
                                        }
                                    }
                                }

                                div {
                                    class: "flex flex-col",
                                    Range {
                                        label: "Trim Ratio",
                                        name: "trim_ratio",
                                        min: 0,
                                        max: 100,
                                        value: prompt.trim_ratio,
                                        help_text: "Percentage of available context to use. Accounts for token counting differences.",
                                        div {
                                            class: "flex justify-between text-xs px-2 mt-1",
                                            span { "0%" }
                                            span { "25%" }
                                            span { "50%" }
                                            span { "75%" }
                                            span { "100%" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Form Actions
                    Card {
                        CardBody {
                            div {
                                class: "flex justify-between",
                                Button {
                                    button_type: ButtonType::Link,
                                    href: crate::routes::prompts::MyAssistants { team_id }.to_string(),
                                    button_scheme: ButtonScheme::Error,
                                    "Cancel"
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    button_scheme: ButtonScheme::Primary,
                                    if prompt.id.is_some() { "Update Assistant" } else { "Create Assistant" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
