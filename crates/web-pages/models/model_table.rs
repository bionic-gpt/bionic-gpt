#![allow(non_snake_case)]
use daisy_rsx::*;
use db::ModelWithPrompt;
use dioxus::prelude::*;

#[component]
pub fn ModelTable(models: Vec<ModelWithPrompt>, team_id: i32) -> Element {
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
                        th {
                            class: "hidden md:block",
                            "Base URL"
                        }
                        th { "Model Type" }

                        th {
                            class: "hidden md:block",
                            "TPM Limit"
                        }
                        th {
                            class: "hidden md:block",
                            "RPM Limit"
                        }
                        th {
                            class: "hidden md:block",
                            "Context Length"
                        }
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
                                        class: "hidden md:block [overflow-wrap:anywhere]",
                                        "{model.base_url}"
                                    }
                                }
                                td {
                                    super::model_type::Model {
                                        model_type: model.model_type
                                    }
                                }
                                td {
                                    class: "hidden md:block",
                                    "{model.tpm_limit}"
                                }
                                td {
                                    class: "hidden md:block",
                                    "{model.rpm_limit}"
                                }
                                td {
                                    class: "hidden md:block",
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
                    }
                }
            }
        }
    )
}
