#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::hero::Hero;
use crate::prompts::prompt_card::PromptCard;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Visibility;
use db::{queries::prompts::Prompt, Category, Dataset, Model};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    categories: Vec<Category>,
    is_saas: bool,
) -> Element {
    // Get categories with more than one prompt
    let categories_with_prompts = get_categories_with_prompts(prompts.clone(), categories.clone());

    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "Assistants",
            header: rsx!(
                h3 { "Assistants" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    modal_trigger: "new-prompt-form",
                    button_scheme: ButtonScheme::Primary,
                    "New Assistant"
                }
            ),

            Hero {
                heading: "Assistants".to_string(),
                subheading: "Discover and create custom chat bots that combine instructions,
                    extra knowledge, and any combination of skills.".to_string()
            }

            div {
                class: "mx-auto max-w-3xl overflow-x-clip px-4",
                TabContainer {
                    if prompts.len() < 20 {
                        // Create an All tab showing everything
                        AssistantTab {
                            checked: true,
                            category: Category {
                                id: -1,
                                name: "All".to_string(),
                                description: "All assistants".to_string()
                            },
                            prompts: prompts.clone(),
                            rbac: rbac.clone(),
                            team_id
                        }
                    }
                    for (index, (category, cat_prompts)) in categories_with_prompts.clone().into_iter().enumerate()  {
                        AssistantTab {
                            checked: prompts.len() >= 20 && index == 0,
                            category,
                            prompts: cat_prompts,
                            rbac: rbac.clone(),
                            team_id
                        }
                    }

                    for item in &prompts {
                        super::delete::DeleteDrawer {
                            team_id: team_id,
                            id: item.id,
                            trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                        }
                    }

                    for prompt in prompts {
                        super::form::Form {
                            id: prompt.id,
                            team_id: team_id,
                            trigger_id: format!("edit-prompt-form-{}", prompt.id),
                            name: prompt.name.clone(),
                            system_prompt: prompt.system_prompt.clone().unwrap_or("".to_string()),
                            datasets: datasets.clone(),
                            selected_dataset_ids: split_datasets(&prompt.selected_datasets),
                            visibility: prompt.visibility,
                            models: models.clone(),
                            categories: categories.clone(),
                            category_id: prompt.category_id,
                            model_id: prompt.model_id,
                            max_history_items: prompt.max_history_items,
                            max_chunks: prompt.max_chunks,
                            max_tokens: prompt.max_tokens,
                            trim_ratio: prompt.trim_ratio,
                            temperature: prompt.temperature.unwrap_or(0.7),
                            description: prompt.description,
                            disclaimer: prompt.disclaimer,
                            example1: prompt.example1,
                            example2: prompt.example2,
                            example3: prompt.example3,
                            example4: prompt.example4,
                            is_saas
                        }
                    }
                }
            }

            // The form to create a model
            super::form::Form {
                team_id: team_id,
                trigger_id: "new-prompt-form".to_string(),
                name: "".to_string(),
                system_prompt: "".to_string(),
                datasets: datasets.clone(),
                selected_dataset_ids: Default::default(),
                models: models.clone(),
                categories: categories.clone(),
                visibility: Visibility::Private,
                model_id: -1,
                category_id: -1,
                max_history_items: 3,
                max_chunks: 10,
                max_tokens: 1024,
                trim_ratio: 80,
                temperature: 0.7,
                description: "".to_string(),
                disclaimer: "LLMs can make mistakes. Check important info.".to_string(),
                example1: None,
                example2: None,
                example3: None,
                example4: None,
                is_saas
            }
        }
    }
}

#[component]
fn AssistantTab(
    category: Category,
    prompts: Vec<Prompt>,
    checked: bool,
    team_id: i32,
    rbac: Rbac,
) -> Element {
    rsx! {
        TabPanel {
            name: "prompt-tabs",
            tab_name: "{category.name}",
            checked,

            div {
                class: "mt-12",
                h3 {
                    class: "text-xl font-semibold md:text-2xl",
                    "{category.name}"
                }
                h4 {
                    class: "mb-8 text-sm md:text-base",
                    "{category.description}"
                }
                div {
                    class: "grid grid-cols-1 gap-x-1.5 gap-y-1 md:gap-x-2 md:gap-y-1.5 lg:grid-cols-2 lg:gap-x-3 lg:gap-y-2.5",

                    for prompt in &prompts {
                        PromptCard {
                            team_id,
                            prompt: prompt.clone(),
                            rbac: rbac.clone()
                        }
                    }
                }
            }
        }
    }
}

// Extracts categories with at least one prompt
fn get_categories_with_prompts(
    prompts: Vec<Prompt>,
    categories: Vec<Category>,
) -> Vec<(Category, Vec<Prompt>)> {
    // Group prompts by category ID
    let mut prompts_by_category: HashMap<i32, Vec<Prompt>> = HashMap::new();
    for prompt in prompts {
        prompts_by_category
            .entry(prompt.category_id)
            .or_default()
            .push(prompt);
    }

    // Create a vector of categories with their associated prompts
    let mut result: Vec<(Category, Vec<Prompt>)> = categories
        .into_iter()
        .filter_map(|category| {
            prompts_by_category
                .get(&category.id)
                .map(|prompts| (category, prompts.clone()))
        })
        .collect();

    // Sort the result by the number of prompts in descending order
    result.sort_by(|(_, prompts_a), (_, prompts_b)| prompts_b.len().cmp(&prompts_a.len()));

    result
}

// Comma separated dataset to vec of i32
fn split_datasets(datasets: &str) -> Vec<i32> {
    let ids: Vec<i32> = datasets
        .split(',')
        .map(|dataset_id| dataset_id.parse::<i32>().unwrap_or(-1))
        .filter(|x| x != &-1)
        .collect();
    ids
}
