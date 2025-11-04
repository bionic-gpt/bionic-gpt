#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::assistants::assistant_card::AssistantCard;
use crate::components::confirm_modal::ConfirmModal;
use crate::i18n;
use crate::routes;
use crate::SectionIntroduction;
use assets::files::*;
use daisy_rsx::{
    Breadcrumb, BreadcrumbItem, Button, ButtonScheme, ButtonType, TabContainer, TabPanel,
};
use db::authz::Rbac;

use db::{queries::prompts::Prompt, Category};
use dioxus::prelude::*;
use std::collections::HashMap;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    prompts: Vec<Prompt>,
    categories: Vec<Category>,
    locale: &str,
) -> String {
    // Get categories with more than one prompt
    let categories_with_prompts = get_categories_with_prompts(prompts.clone(), categories.clone());
    let assistants_label = i18n::assistants(locale);
    let assistant_label = i18n::assistant(locale);

    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: assistants_label.clone(),
            locale: locale.to_string(),
            header: rsx!(
                Breadcrumb {
                    items: vec![BreadcrumbItem {
                        text: assistants_label.clone(),
                        href: None
                    }]
                }
                div {
                    a {
                        href: crate::routes::prompts::MyAssistants{team_id}.to_string(),
                        class: "btn btn-ghost btn-sm font-bold! mr-4",
                        {format!("My {}", assistants_label.clone())}
                    }
                    Button {
                        button_type: ButtonType::Link,
                        prefix_image_src: "{button_plus_svg.name}",
                        href: routes::prompts::New{team_id}.to_string(),
                        button_scheme: ButtonScheme::Primary,
                        {format!("New {}", assistant_label.clone())}
                    }
                }
            ),

            div {
                class: "mx-auto max-w-3xl overflow-x-clip px-4",

                SectionIntroduction {
                    header: assistants_label.clone(),
                    subtitle: "Discover and create custom chat bots that combine instructions,
                    extra knowledge, and any combination of skills.".to_string(),
                    is_empty: prompts.is_empty(),
                    empty_text: format!(
                        "You haven't created any {} yet.",
                        assistants_label.to_lowercase()
                    ),
                }

                if ! prompts.is_empty() {
                    TabContainer {
                        class: "mt-5 w-full",
                        if prompts.len() < 20 {
                            // Create an All tab showing everything
                            AssistantTab {
                                checked: true,
                                category: Category {
                                    id: -1,
                                    name: format!("All {}", assistants_label.clone()),
                                    description: format!("All {}", assistants_label.to_lowercase())
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
                            ConfirmModal {
                                action: crate::routes::prompts::Delete{team_id, id: prompt.id}.to_string(),
                                trigger_id: format!("delete-trigger-{}-{}", prompt.id, team_id),
                                submit_label: "Delete".to_string(),
                                heading: format!("Delete this {}?", assistant_label.clone()),
                                warning: format!(
                                    "Are you sure you want to delete this {}?",
                                    assistant_label.clone()
                                ),
                                hidden_fields: vec![
                                    ("team_id".into(), team_id.to_string()),
                                    ("id".into(), prompt.id.to_string()),
                                ],
                            }

                            super::view_prompt::ViewDrawer {
                                team_id: team_id,
                                prompt: prompt.clone(),
                                trigger_id: format!("view-trigger-{}-{}", prompt.id, team_id)
                            }
                        }
                    }
                }
            }


        }
    };

    crate::render(page)
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
                class: "w-full",
                div {
                    class: "space-y-2",
                    for prompt in &prompts {
                        AssistantCard {
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
