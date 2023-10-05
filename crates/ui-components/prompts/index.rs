use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::{queries::prompts::Prompt, Dataset, Model};
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
                            primary_action: (
                                "New Prompt Template", 
                                crate::routes::prompts::new_route(cx.props.organisation_id)
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
                                DataTable {
                                    table {
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
                                                            "{prompt.updated_at}"
                                                        }
                                                        td {
                                                            class: "text-right",
                                                            DropDown {
                                                                direction: Direction::West,
                                                                button_text: "...",
                                                                DropDownLink {
                                                                    href: "{crate::routes::prompts::edit_route(cx.props.organisation_id, prompt.id)}",
                                                                    target: "_top",
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
                        }

                        // The form to create a model
                        super::form::Form {
                            organisation_id: cx.props.organisation_id,
                            trigger_id: "new-prompt-form".to_string(),
                            name: "".to_string(),
                            template: "Context information is below.
--------------------
{context_str}
--------------------".to_string(),
                            datasets: cx.props.datasets.clone(),
                            models: cx.props.models.clone(),
                            model_id: -1,
                            min_history_items: 1,
                            max_history_items: 3,
                            min_chunks: 3,
                            max_chunks: 10,
                            max_tokens: 1024,
                            temperature: 0.7,
                            top_p: 0.0,
                        }
                    })
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
