use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use db::queries::prompts::Prompt;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    prompts: Vec<Prompt>,
}

pub fn index(organisation_id: i32, prompts: Vec<Prompt>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Prompts,
                team_id: cx.props.organisation_id,
                title: "Prompts",
                header: cx.render(rsx!(
                    h3 { "Prompts" }
                    a {
                        class: "btn btn-primary",
                        href: "{crate::routes::prompts::new_route(cx.props.organisation_id)}",
                        "New Prompt Template"
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
        },
    ))
}
