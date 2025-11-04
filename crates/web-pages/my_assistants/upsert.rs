#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Category, Prompt, Visibility};
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
    pub models: Vec<Prompt>,
}

pub fn page(
    team_id: i32,
    rbac: Rbac,
    prompt: PromptForm,
    show_company_visibility: bool,
    locale: &str,
) -> String {
    let example1 = prompt.example1.clone().unwrap_or_default();
    let example2 = prompt.example2.clone().unwrap_or_default();
    let example3 = prompt.example3.clone().unwrap_or_default();
    let example4 = prompt.example4.clone().unwrap_or_default();
    let assistant_label = i18n::assistant(locale);
    let assistants_label = i18n::assistants(locale);
    let action_label = if prompt.id.is_some() {
        format!("Edit {}", assistant_label.clone())
    } else {
        format!("Create {}", assistant_label.clone())
    };

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: assistant_label.clone(),
            locale: Some(locale.to_string()),
            header: rsx!(
                Breadcrumb {
                    items: vec![
                        BreadcrumbItem {
                            text: assistants_label.clone(),
                            href: Some(crate::routes::prompts::Index{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: format!("My {}", assistants_label.clone()),
                            href: Some(crate::routes::prompts::MyAssistants{team_id}.to_string())
                        },
                        BreadcrumbItem {
                            text: action_label.clone(),
                            href: None
                        }
                    ]
                }
                h3 {
                    {action_label.clone()}
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
                            title: format!("{} Details", assistant_label.clone())
                        }
                        CardBody {
                            div {
                                class: "flex flex-col gap-6",

                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {
                                        class: "flex flex-col",
                                       Fieldset {
                                            legend: format!("{} Name", assistant_label.clone()),
                                            help_text: "Make the name memorable and imply its usage.",
                                            Input {
                                                input_type: InputType::Text,
                                                name: "name",
                                                value: prompt.name,
                                                required: true
                                            }
                                        }
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Category",
                                           help_text: format!(
                                                "Categories help users find {}.",
                                                assistants_label.to_lowercase()
                                            ),
                                            Select {
                                                name: "category_id",
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
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Visibility",
                                           help_text: format!(
                                                "Set to private if you don't want to share this {}.",
                                                assistant_label.to_lowercase()
                                            ),
                                            Select {
                                                name: "visibility",
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
                                                if show_company_visibility {
                                                    SelectOption {
                                                        value: "{crate::visibility_to_string(Visibility::Company)}",
                                                        selected_value: "{prompt.visibility}",
                                                        {crate::visibility_to_string(Visibility::Company)}
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Model",
                                            help_text: "The model will be used to answer any questions.",
                                            Select {
                                                name: "model_id",
                                                value: "{prompt.model_id}",
                                                required: true,
                                                for model_prompt in &prompt.models {
                                                    SelectOption {
                                                        value: "{model_prompt.model_id}",
                                                        selected_value: "{prompt.model_id}",
                                                        "{model_prompt.name}"
                                                    }
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
                                    help_text: format!(
                                        "Describe what this {} does and how it can help users.",
                                        assistant_label.to_lowercase()
                                    ),
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
                                help_text: format!(
                                    "Define the {}'s behavior, personality, and capabilities.",
                                    assistant_label.to_lowercase()
                                ),
                                "{prompt.system_prompt}",
                            }
                        }
                    }

                    // Assistant Icon Section
                    Card {
                        class: "mb-6",
                        CardHeader {
                            title: format!("{} Icon", assistant_label.clone())
                        }
                        CardBody {
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-2",
                                    {format!("Upload {} Icon", assistant_label.clone())}
                                }
                                p {
                                    class: "text-sm text-gray-500 mb-3",
                                    {format!(
                                        "Choose an image that represents your {}. Recommended size: 48x48 pixels.",
                                        assistant_label.to_lowercase()
                                    )}
                                }
                                FileInput {
                                    name: "image_icon",
                                    accept: "image/*",
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
                                Fieldset {
                                    legend: "Disclaimer",
                                    help_text: "This is displayed at the bottom of the chat.",
                                    Input {
                                        input_type: InputType::Text,
                                        name: "disclaimer",
                                        value: "{prompt.disclaimer}"
                                    }
                                }
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Example 1",
                                        help_text: "Give the user an example prompt.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "example1",
                                            value: "{example1}"
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Example 2",
                                        help_text: "Give the user an example prompt.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "example2",
                                            value: "{example2}"
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Example 3",
                                        help_text: "Give the user an example prompt.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "example3",
                                            value: "{example3}"
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-col",
                                    Fieldset {
                                        legend: "Example 4",
                                        help_text: "Give the user an example prompt.",
                                        Input {
                                            input_type: InputType::Text,
                                            name: "example4",
                                            value: "{example4}"
                                        }
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
                                        Fieldset {
                                            legend: "Temperature",
                                            help_text: "Value between 0 and 2. Higher values make output more random.",
                                            Input {
                                                input_type: InputType::Number,
                                                step: "0.1",
                                                name: "temperature",
                                                value: "{prompt.temperature}",
                                                required: true
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Max History Items",
                                            help_text: "How much conversation history to include. Set to zero for no history.",
                                            Input {
                                                input_type: InputType::Number,
                                                name: "max_history_items",
                                                value: "{prompt.max_history_items}",
                                                required: true
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Max Tokens",
                                            help_text: "Context space reserved for the LLM's reply.",
                                            Input {
                                                input_type: InputType::Number,
                                                name: "max_tokens",
                                                value: "{prompt.max_tokens}",
                                                required: true
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex flex-col",
                                        Fieldset {
                                            legend: "Max Chunks",
                                            help_text: "Maximum number of dataset chunks to include.",
                                            Input {
                                                input_type: InputType::Number,
                                                name: "max_chunks",
                                                value: "{prompt.max_chunks}",
                                                required: true
                                            }
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
                                    {
                                        if prompt.id.is_some() {
                                            format!("Update {}", assistant_label.clone())
                                        } else {
                                            format!("Create {}", assistant_label.clone())
                                        }
                                    }
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
