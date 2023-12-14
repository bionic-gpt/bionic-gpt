#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::{queries::prompts::Prompt, Dataset, DatasetConnection, Model, Visibility};
use dioxus::prelude::*;

#[inline_props]
pub fn Page(
    cx: Scope,
    team_id: i32,
    is_sys_admin: bool,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Prompts,
            team_id: *team_id,
            is_sys_admin: *is_sys_admin,
            title: "Prompts",
            header: cx.render(rsx!(
                h3 { "Prompts" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-prompt-form",
                    button_scheme: ButtonScheme::Primary,
                    "New Prompt"
                }
            ))

            if prompts.is_empty() {
                cx.render(rsx! {
                    BlankSlate {
                        heading: "Looks like you haven't configured any prompts yet",
                        visual: nav_dashboard_svg.name,
                        description: "Researchers use prompt engineering to improve the capacity of LLMs on a wide range of common and complex tasks such as question answering and arithmetic reasoning.",
                        primary_action_drawer: (
                            "New Prompt",
                            "new-prompt-form", 
                        )
                    }
                })
            } else {

                cx.render(rsx! {
                    Box {
                        class: "has-data-table",
                        BoxHeader {
                            title: "Prompts"
                        }
                        BoxBody {
                            table {
                                class: "table table-sm",
                                thead {
                                    th { "Name" }
                                    th { "Dataset(s)" }
                                    th { "Visibility" }
                                    th { "Model" }
                                    th { "Updated" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {

                                    prompts.iter().map(|prompt| {
                                        cx.render(rsx!(
                                            tr {
                                                td {
                                                    "{prompt.name}"
                                                }
                                                td {
                                                    super::dataset_connection::DatasetConnection {
                                                        connection: prompt.dataset_connection
                                                    }
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
                                                                prompt.id, *team_id),
                                                            href: "#",
                                                            target: "_top",
                                                            "Delete"
                                                        }
                                                    }
                                                }
                                            }
                                        ))
                                    })
                                }
                            }
                        }
                    }

                    prompts.iter().map(|item| rsx!(
                        cx.render(rsx!(
                            super::delete::DeleteDrawer {
                                team_id: *team_id,
                                id: item.id,
                                trigger_id: format!("delete-trigger-{}-{}", item.id, *team_id)
                            }
                        ))
                    ))

                    prompts.iter().map(|prompt| {
                        // The form to edit a prompt
                        cx.render(rsx!(
                            super::form::Form {
                                id: prompt.id,
                                team_id: *team_id,
                                trigger_id: format!("edit-prompt-form-{}", prompt.id),
                                name: prompt.name.clone(),
                                system_prompt: prompt.system_prompt.clone().unwrap_or("".to_string()),
                                datasets: datasets.clone(),
                                selected_dataset_ids: split_datasets(&prompt.selected_datasets),
                                dataset_connection: prompt.dataset_connection,
                                visibility: prompt.visibility,
                                models: models.clone(),
                                model_id: prompt.model_id,
                                max_history_items: prompt.max_history_items,
                                max_chunks: prompt.max_chunks,
                                max_tokens: prompt.max_tokens,
                                trim_ratio: prompt.trim_ratio,
                                temperature: prompt.temperature.unwrap_or(0.7),
                                top_p: prompt.top_p.unwrap_or(0.0),
                            }
                        ))
                    })
                })
            }

            // The form to create a model
            super::form::Form {
                team_id: *team_id,
                trigger_id: "new-prompt-form".to_string(),
                name: "".to_string(),
                system_prompt: "".to_string(),
                datasets: datasets.clone(),
                dataset_connection: DatasetConnection::None,
                selected_dataset_ids: Default::default(),
                models: models.clone(),
                visibility: Visibility::Private,
                model_id: -1,
                max_history_items: 3,
                max_chunks: 10,
                max_tokens: 1024,
                trim_ratio: 80,
                temperature: 0.7,
                top_p: 0.0,
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
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
