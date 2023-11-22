use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::{queries::prompts::Prompt, Dataset, DatasetConnection, Model, Visibility};
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
}

pub fn index(
    organisation_id: i32,
    prompts: Vec<Prompt>,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::Prompts,
                team_id: cx.props.organisation_id,
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

                if cx.props.prompts.is_empty() {
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

                                        cx.props.prompts.iter().map(|prompt| {
                                            cx.render(rsx!(
                                                tr {
                                                    td {
                                                        "{prompt.name}"
                                                    }
                                                    td {
                                                        super::dataset_connection::DatasetConnection {
                                                            connection: prompt.dataset_connection,
                                                            datasets: prompt.datasets.clone()
                                                        }
                                                    }
                                                    td {
                                                        super::visibility::VisLabel {
                                                            visibility: &prompt.visibility
                                                        }
                                                    }
                                                    td {
                                                        "{prompt.model_name}"
                                                    }
                                                    td {
                                                        RelativeTime {
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
                                                        }
                                                    }
                                                }
                                            ))
                                        })
                                    }
                                }
                            }
                        }

                        cx.props.prompts.iter().map(|prompt| {
                            // The form to edit a prompt
                            cx.render(rsx!(
                                super::form::Form {
                                    id: prompt.id,
                                    organisation_id: cx.props.organisation_id,
                                    trigger_id: format!("edit-prompt-form-{}", prompt.id),
                                    name: prompt.name.clone(),
                                    system_prompt: prompt.system_prompt.clone().unwrap_or("".to_string()),
                                    datasets: cx.props.datasets.clone(),
                                    selected_dataset_ids: split_datasets(&prompt.selected_datasets),
                                    dataset_connection: prompt.dataset_connection,
                                    visibility: prompt.visibility,
                                    models: cx.props.models.clone(),
                                    model_id: prompt.model_id,
                                    max_history_items: prompt.max_history_items,
                                    max_chunks: prompt.max_chunks,
                                    max_tokens: prompt.max_tokens,
                                    temperature: prompt.temperature.unwrap_or(0.7),
                                    top_p: prompt.top_p.unwrap_or(0.0),
                                }
                            ))
                        })
                    })
                }

                // The form to create a model
                super::form::Form {
                    organisation_id: cx.props.organisation_id,
                    trigger_id: "new-prompt-form".to_string(),
                    name: "".to_string(),
                    system_prompt: "".to_string(),
                    datasets: cx.props.datasets.clone(),
                    dataset_connection: DatasetConnection::None,
                    selected_dataset_ids: Default::default(),
                    models: cx.props.models.clone(),
                    visibility: Visibility::Private,
                    model_id: -1,
                    max_history_items: 3,
                    max_chunks: 10,
                    max_tokens: 1024,
                    temperature: 0.7,
                    top_p: 0.0,
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        Props {
            organisation_id,
            prompts,
            datasets,
            models,
        },
    ))
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
