#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::my_assistants::{
    advanced_settings_card::AdvancedSettingsCard, assistant_details_card::AssistantDetailsCard,
    datasets_card::DatasetsCard, examples_card::ExamplesCard, integrations_card::IntegrationsCard,
    system_prompt_card::SystemPromptCard,
};
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    prompt: db::SinglePrompt,
    datasets: Vec<db::PromptDataset>,
    integrations: Vec<db::PromptIntegration>,
) -> String {
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
                        href: crate::routes::prompts::Edit{team_id, prompt_id: prompt.id}.to_string(),
                        "Edit"
                    }
                    Button {
                        button_scheme: ButtonScheme::Error,
                        button_size: ButtonSize::Small,
                        popover_target: format!("delete-trigger-{}-{}", prompt.id, team_id),
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
                        div {
                            class: "flex-shrink-0",
                            Avatar {
                                avatar_size: AvatarSize::Large,
                                avatar_type: AvatarType::User
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

                // Replace all card sections with component calls
                AssistantDetailsCard { prompt: prompt.clone() }
                SystemPromptCard { system_prompt: prompt.system_prompt.clone().unwrap_or_default() }
                DatasetsCard { team_id, prompt_id: prompt.id, datasets: datasets.clone() }
                IntegrationsCard { team_id, prompt_id: prompt.id, integrations: integrations.clone() }
                ExamplesCard {
                    disclaimer: prompt.disclaimer.clone(),
                    example1, example2, example3, example4
                }
                AdvancedSettingsCard { prompt: prompt.clone() }
            }
        }

        // Delete confirmation modal
        crate::ConfirmModal {
            action: crate::routes::prompts::Delete{team_id, id: prompt.id}.to_string(),
            trigger_id: format!("delete-trigger-{}-{}", prompt.id, team_id),
            submit_label: "Delete".to_string(),
            heading: "Delete this Assistant?".to_string(),
            warning: "Are you sure you want to delete this Assistant?".to_string(),
            hidden_fields: vec![
                ("team_id".into(), team_id.to_string()),
                ("id".into(), prompt.id.to_string()),
            ],
        }
    };

    crate::render(page)
}
