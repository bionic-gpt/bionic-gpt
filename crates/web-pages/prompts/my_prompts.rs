#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::hero::Hero;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Visibility;
use db::{queries::prompts::Prompt, Category, Dataset, Model};
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    categories: Vec<Category>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "My Assistants",
            header: rsx!(
                h3 {
                    span {
                        class: "hidden md:block",
                        "My Assistants"
                    }
                }
                div {
                    a {
                        href: crate::routes::prompts::Index{team_id}.to_string(),
                        class: "btn btn-ghost btn-sm !font-bold mr-4",
                        "Explore Assistants"
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
                heading: "Your Assistants".to_string(),
                subheading: "Discover and create custom chat bots that combine instructions,
                    extra knowledge, and any combination of skills.".to_string()
            }

            Card {
                class: "has-data-table max-w-[64rem] mx-auto",
                CardHeader {
                    title: "My Assistants"
                }
                CardBody {
                    table {
                        class: "table table-sm table-layout-fixed",
                        thead {
                            th {
                                class: "hidden md:block",
                                "Last Updated"
                            }
                            th {
                                class: "w-full",
                                "Name"
                            }
                            th { "Visibility" }
                            th {
                                "Edit"
                            }
                            th {
                                class: "hidden md:block",
                                class: "text-right",
                                "Action"
                            }
                        }
                        tbody {
                            for prompt in &prompts {
                                tr {
                                    td {
                                        class: "hidden md:block",
                                        RelativeTime {
                                            format: RelativeTimeFormat::Relative,
                                            datetime: "{prompt.updated_at}"
                                        }
                                    }
                                    td {
                                        strong {
                                            "{prompt.name}"
                                        }
                                    }
                                    td {
                                        super::visibility::VisLabel {
                                            visibility: prompt.visibility
                                        }
                                    }
                                    td {
                                        Button {
                                            modal_trigger: format!("edit-prompt-form-{}", prompt.id),
                                            button_scheme: ButtonScheme::Default,
                                            "Edit"
                                        }

                                    }
                                    td {
                                        class: "text-right hidden md:block",
                                        DropDown {
                                            direction: Direction::Left,
                                            button_text: "...",
                                            DropDownLink {
                                                drawer_trigger: format!("delete-trigger-{}-{}",
                                                    prompt.id, team_id),
                                                href: "#",
                                                target: "_top",
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
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
                    can_make_assistant_public: rbac.can_make_assistant_public()
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
                can_make_assistant_public: rbac.can_make_assistant_public()
            }
        }
    };

    crate::render(page)
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
