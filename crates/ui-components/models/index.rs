use crate::app_layout::{Layout, SideBar};
use db::queries::models::Model;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    models: Vec<Model>,
}

pub fn index(organisation_id: i32, models: Vec<Model>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::Models,
                team_id: cx.props.organisation_id,
                title: "Models",
                header: cx.render(rsx!(
                    h3 { "Models" }
                    a {
                        class: "btn btn-primary",
                        href: "{crate::routes::models::new_route(cx.props.organisation_id)}",
                        "New Model"
                    }
                )),

                Box {
                    class: "has-data-table",
                    BoxHeader {
                        title: "Models"
                    }
                    BoxBody {
                        DataTable {
                            table {
                                thead {
                                    th { "Name" }
                                    th { "Base URL" }
                                    th { "Parameters" }
                                    th { "Context Length" }
                                    th {
                                        class: "text-right",
                                        "Action"
                                    }
                                }
                                tbody {

                                    cx.props.models.iter().map(|model| {
                                        cx.render(rsx!(
                                            tr {
                                                td {
                                                    "{model.name}"
                                                }
                                                td {
                                                    "{model.base_url}"
                                                }
                                                td {
                                                    "{model.billion_parameters} Billion"
                                                }
                                                td {
                                                    "{model.context_size_bytes} Bytes"
                                                }
                                                td {
                                                    class: "text-right",
                                                    DropDown {
                                                        direction: Direction::West,
                                                        button_text: "...",
                                                        DropDownLink {
                                                            href: "{crate::routes::models::edit_route(cx.props.organisation_id, model.id)}",
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
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        Props {
            organisation_id,
            models,
        },
    ))
}
