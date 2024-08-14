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
                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "Assistants"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Name" }
                                th { "Visibility" }
                                th { "Model" }
                                th { "Updated" }
                                th {
                                    class: "text-right",
                                    "Action"
                                }
                            }
                            tbody {
                                for prompt in &prompts {
                                    tr {
                                        td {
                                            "{prompt.name}"
                                        }
                                        td {
                                            super::visibility::VisLabel {
                                                visibility: prompt.visibility
                                            }
                                        }
                                        td {
                                            "{prompt.model_name}"
                                        }
                                        td {
                                            RelativeTime {
                                                format: RelativeTimeFormat::Relative,
                                                datetime: "{prompt.updated_at}"
                                            }
                                        }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    href: "#",
                                                    drawer_trigger: format!("edit-prompt-form-{}", prompt.id),
                                                    "Edit"
                                                }
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
                        model_id: prompt.model_id,
                        max_history_items: prompt.max_history_items,
                        max_chunks: prompt.max_chunks,
                        max_tokens: prompt.max_tokens,
                        trim_ratio: prompt.trim_ratio,
                        temperature: prompt.temperature.unwrap_or(0.7),
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
