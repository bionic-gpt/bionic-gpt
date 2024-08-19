#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::Visibility;
use db::{queries::prompts::Prompt, Dataset, Model};
use dioxus::prelude::*;

#[component]
pub fn Page(
    team_id: i32,
    rbac: Rbac,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    is_saas: bool,
) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Prompts,
            team_id: team_id,
            rbac: rbac,
            title: "Assistants",
            header: rsx!(
                h3 { "Assistants" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-prompt-form",
                    button_scheme: ButtonScheme::Primary,
                    "New Assistant"
                }
            )

            if prompts.is_empty() {
                BlankSlate {
                    heading: "Looks like you haven't configured any assistants yet",
                    visual: nav_dashboard_svg.name,
                    description: "AI Assistants use your data to help you with your work.",
                    primary_action_drawer: (
                        "New Assistant".to_string(),
                        "new-prompt-form".to_string(),
                    )
                }
            } else {

                div {
                    class: "grid md:grid-cols-3 xl:grid-cols-4 sm:grid-cols-1 gap-4",
                    for prompt in &prompts {
                        Box {
                            BoxHeader {
                                class: "truncate ellipses flex justify-between",
                                title: "{prompt.name}",
                                super::visibility::VisLabel {
                                    visibility: prompt.visibility
                                }
                            }
                            BoxBody {
                                p {
                                    class: "text-sm",
                                    "{prompt.description}"
                                }
                                div {
                                    class: "mt-3 flex flex-row justify-between",
                                    a {
                                        class: "btn btn-primary btn-sm",
                                        href: crate::routes::prompts::NewChat{team_id, prompt_id: prompt.id}.to_string(),
                                        "Chat"
                                    }
                                    div {
                                        class: "flex gap-1",
                                        Button {
                                            drawer_trigger: format!("delete-trigger-{}-{}", prompt.id, team_id),
                                            button_scheme: ButtonScheme::Danger,
                                            "Delete"
                                        }
                                        Button {
                                            drawer_trigger: format!("edit-prompt-form-{}", prompt.id),
                                            "Edit"
                                        }
                                    }
                                }
                                div {
                                    class: "mt-3 text-xs flex justify-center gap-1",
                                    "Last update",
                                    RelativeTime {
                                        format: RelativeTimeFormat::Relative,
                                        datetime: "{prompt.updated_at}"
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

            // The form to create a model
            super::form::Form {
                team_id: team_id,
                trigger_id: "new-prompt-form".to_string(),
                name: "".to_string(),
                system_prompt: "".to_string(),
                datasets: datasets.clone(),
                selected_dataset_ids: Default::default(),
                models: models.clone(),
                visibility: Visibility::Private,
                model_id: -1,
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

// Comma seperated dataset to vec of i32
fn split_datasets(datasets: &str) -> Vec<i32> {
    let ids: Vec<i32> = datasets
        .split(',')
        .map(|dataset_id| dataset_id.parse::<i32>().unwrap_or(-1))
        .filter(|x| x != &-1)
        .collect();
    ids
}
