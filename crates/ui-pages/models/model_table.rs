#![allow(non_snake_case)]
use dioxus::prelude::*;
use daisy_rsx::*; 
use db::Model;

#[component]
pub fn ModelTable(models: Vec<Model>, team_id: i32) -> Element {
    rsx!(
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
                        for model in &models {
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
                                        DropDownLink {
                                            drawer_trigger: format!("delete-trigger-{}-{}",
                                                model.id, team_id),
                                            href: "#",
                                            target: "_top",
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }

                        for item in models {
                            super::delete::DeleteDrawer {
                                team_id: team_id,
                                id: item.id,
                                trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                            }
                        }

                    }
                }
            }
        }
    )
}
