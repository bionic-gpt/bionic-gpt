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
                div {
                    a {
                        href: crate::routes::prompts::MyPrompts{team_id}.to_string(),
                        class: "mr-4",
                        "My Assistants"
                    }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        modal_trigger: "new-prompt-form",
                        button_scheme: ButtonScheme::Primary,
                        "New Assistant"
                    }
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
                    class: "w-full",
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

                    for prompt in &prompts {
                        super::delete::DeleteDrawer {
                            team_id: team_id,
                            id: prompt.id,
                            trigger_id: format!("delete-trigger-{}-{}", prompt.id, team_id)
                        }

                        super::view_prompt::ViewDrawer {
                            team_id: team_id,
                            prompt: prompt.clone(),
                            trigger_id: format!("view-trigger-{}-{}", prompt.id, team_id)
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
                class: "mt-12  w-full",
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
