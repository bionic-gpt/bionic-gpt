#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use db::queries::models::Model;
use db::ModelType;
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn Page(cx: Scope<Props>, organisation_id: i32, models: Vec<Model>) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Models,
            team_id: *organisation_id,
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
                    table {
                        class: "table table-sm",
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

                            models.iter().map(|model| {
                                cx.render(rsx!(
                                    tr {
                                        td {
                                            strong {
                                                "{model.name}"
                                            }
                                        }
                                        td {
                                            code {
                                                "{model.base_url}"
                                            }
                                        }
                                        td {
                                            super::model_type::Model {
                                                model_type: model.model_type
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
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    href: "#",
                                                    drawer_trigger: format!("edit-model-form-{}", model.id),
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

            // The form to create a model
            super::form::Form {
                organisation_id: *organisation_id,
                trigger_id: "new-model-form".to_string(),
                name: "".to_string(),
                model_type: "LLM".to_string(),
                base_url: "".to_string(),
                billion_parameters: 7,
                api_key: "".to_string(),
                context_size_bytes: 2048,
            }


            models.iter().map(|model| {
                // The form to edit a model
                let model_type = if model.model_type == ModelType::LLM {
                    "LLM"
                } else {
                    "Embeddings"
                };
                cx.render(rsx!(
                    super::form::Form {
                        id: model.id,
                        organisation_id: *organisation_id,
                        trigger_id: format!("edit-model-form-{}", model.id),
                        name: model.name.clone(),
                        model_type: model_type.to_string(),
                        base_url: model.base_url.clone(),
                        api_key: model.api_key.clone().unwrap_or("".to_string()),
                        billion_parameters: model.billion_parameters,
                        context_size_bytes: model.context_size,
                    }
                ))
            })
        }
    })
}
pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
