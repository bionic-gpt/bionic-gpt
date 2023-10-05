use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
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
                section_class: "normal",
                selected_item: SideBar::Models,
                team_id: cx.props.organisation_id,
                title: "Models",
                header: cx.render(rsx!(
                    h3 { "Models" }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: "new-model-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Model"
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
                                    th { "Model Type" }
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
                                                    super::model_type::Model {
                                                        model_type: &model.model_type
                                                    }
                                                }
                                                td {
                                                    "{model.billion_parameters} Billion"
                                                }
                                                td {
                                                    "{model.context_size}"
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

            // The form to create an invitation
            super::form::Form {
                organisation_id: cx.props.organisation_id,
                trigger_id: "new-model-form".to_string(),
                name: "".to_string(),
                base_url: "".to_string(),
                billion_parameters: 7,
                context_size_bytes: 2048,
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
