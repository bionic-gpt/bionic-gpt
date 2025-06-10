#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(team_id: i32, rbac: Rbac, prompt: super::upsert::PromptForm) -> String {
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
            title: "Assistant Details",
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
                            text: prompt.name.clone(),
                            href: None
                        }
                    ]
                }
                div {
                    class: "flex gap-2",
                    Button {
                        button_type: ButtonType::Link,
                        button_scheme: ButtonScheme::Neutral,
                        button_size: ButtonSize::Small,
                        href: crate::routes::prompts::Edit{team_id, prompt_id: prompt.id.unwrap_or(0)}.to_string(),
                        "Edit"
                    }
                    Button {
                        button_scheme: ButtonScheme::Error,
                        button_size: ButtonSize::Small,
                        popover_target: format!("delete-trigger-{}-{}", prompt.id.unwrap_or(0), team_id),
                        "Delete"
                    }
                }
            ),

            div {
                class: "p-4 max-w-4xl w-full mx-auto",

                // Assistant Header
                div {
                    class: "mb-8",
                    div {
                        class: "flex items-center gap-4 mb-4",
                        if let Some(_object_id) = prompt.id {
                            // Try to show image if available - this would need the actual object_id from the prompt
                            div {
                                class: "flex-shrink-0",
                                Avatar {
                                    avatar_size: AvatarSize::Large,
                                    avatar_type: AvatarType::User
                                }
                            }
                        } else {
                            div {
                                class: "flex-shrink-0",
                                Avatar {
                                    avatar_size: AvatarSize::Large,
                                    avatar_type: AvatarType::User
                                }
                            }
                        }
                        div {
                            h1 {
                                class: "text-3xl font-bold text-gray-900",
                                "{prompt.name}"
                            }
                            p {
                                class: "text-lg text-gray-600 mt-1",
                                "{prompt.description}"
                            }
                        }
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
                            class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Assistant Name"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.name}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Category"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    {
                                        prompt.categories.iter()
                                            .find(|c| c.id == prompt.category_id)
                                            .map(|c| c.name.as_str())
                                            .unwrap_or("Unknown")
                                    }
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Visibility"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.visibility}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Model"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    {
                                        prompt.models.iter()
                                            .find(|m| m.id == prompt.model_id)
                                            .map(|m| m.name.as_str())
                                            .unwrap_or("Unknown")
                                    }
                                }
                            }
                        }

                        div {
                            class: "mt-6",
                            label {
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Description"
                            }
                            p {
                                class: "text-sm text-gray-900 bg-gray-50 p-3 rounded border",
                                "{prompt.description}"
                            }
                        }
                    }
                }

                // System Prompt Section
                Card {
                    class: "mb-6",
                    CardHeader {
                        title: "System Prompt"
                    }
                    CardBody {
                        div {
                            class: "bg-gray-50 p-4 rounded border font-mono text-sm leading-relaxed whitespace-pre-wrap",
                            "{prompt.system_prompt}"
                        }
                    }
                }

                // Datasets Section
                Card {
                    class: "mb-6",
                    div {
                        class: "card-header flex justify-between items-center p-4 border-b",
                        h3 {
                            class: "text-lg font-semibold",
                            "Connected Datasets"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            href: crate::routes::prompts::ManageDatasets{team_id, prompt_id: prompt.id.unwrap_or(0)}.to_string(),
                            button_scheme: ButtonScheme::Primary,
                            button_size: ButtonSize::Small,
                            "Manage Datasets"
                        }
                    }
                    CardBody {
                        p {
                            class: "text-gray-600",
                            "Click 'Manage Datasets' to view and configure dataset connections for this assistant."
                        }
                    }
                }

                // Integrations Section
                Card {
                    class: "mb-6",
                    div {
                        class: "card-header flex justify-between items-center p-4 border-b",
                        h3 {
                            class: "text-lg font-semibold",
                            "Connected Integrations"
                        }
                        Button {
                            button_type: ButtonType::Link,
                            href: crate::routes::prompts::ManageIntegrations{team_id, prompt_id: prompt.id.unwrap_or(0)}.to_string(),
                            button_scheme: ButtonScheme::Primary,
                            button_size: ButtonSize::Small,
                            "Manage Integrations"
                        }
                    }
                    CardBody {
                        p {
                            class: "text-gray-600",
                            "Click 'Manage Integrations' to view and configure integration connections for this assistant."
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
                        div {
                            class: "space-y-4",

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Disclaimer"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.disclaimer}"
                                }
                            }

                            if !example1.is_empty() {
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700 mb-1",
                                        "Example 1"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                        "{example1}"
                                    }
                                }
                            }

                            if !example2.is_empty() {
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700 mb-1",
                                        "Example 2"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                        "{example2}"
                                    }
                                }
                            }

                            if !example3.is_empty() {
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700 mb-1",
                                        "Example 3"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                        "{example3}"
                                    }
                                }
                            }

                            if !example4.is_empty() {
                                div {
                                    label {
                                        class: "block text-sm font-medium text-gray-700 mb-1",
                                        "Example 4"
                                    }
                                    p {
                                        class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                        "{example4}"
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
                            class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Temperature"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.temperature}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Max History Items"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.max_history_items}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Max Tokens"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.max_tokens}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Max Chunks"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.max_chunks}"
                                }
                            }

                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Trim Ratio"
                                }
                                p {
                                    class: "text-sm text-gray-900 bg-gray-50 p-2 rounded border",
                                    "{prompt.trim_ratio}%"
                                }
                            }
                        }
                    }
                }
            }
        }

        // Delete confirmation modal
        if let Some(id) = prompt.id {
            crate::ConfirmModal {
                action: crate::routes::prompts::Delete{team_id, id}.to_string(),
                trigger_id: format!("delete-trigger-{}-{}", id, team_id),
                submit_label: "Delete".to_string(),
                heading: "Delete this Assistant?".to_string(),
                warning: "Are you sure you want to delete this Assistant?".to_string(),
                hidden_fields: vec![
                    ("team_id".into(), team_id.to_string()),
                    ("id".into(), id.to_string()),
                ],
            }
        }
    };

    crate::render(page)
}
