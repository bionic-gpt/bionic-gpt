#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Integration;
use dioxus::prelude::*;
use integrations::IntegrationTool;

#[component]
pub fn IntegrationTable(
    integrations: Vec<Integration>,
    tool_integrations: Vec<IntegrationTool>,
    team_id: i32,
) -> Element {
    rsx!(
        Card {
            class: "has-data-table",
            CardHeader {
                title: "Integrations"
            }
            CardBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Name" }
                        th {
                            class: "max-sm:hidden",
                            "Status"
                        }
                        th { "Integration Type" }

                        th {
                            class: "text-right",
                            "Action"
                        }
                    }
                    tbody {
                        for (index, integration) in tool_integrations.iter().enumerate() {
                            tr {
                                td {
                                    strong {
                                        "{integration.title}"
                                    }
                                }
                                td {

                                }
                                td {
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            href: "#",
                                            drawer_trigger: format!("show-builtin-integration-details-{}", index),
                                            "Show Details"
                                        }
                                    }
                                }
                            }
                        }
                        for integration in &integrations {
                            tr {
                                td {
                                    strong {
                                        "{integration.name}"
                                    }
                                }
                                td {
                                    super::status_type::Status {
                                        integration_status: integration.integration_status
                                    }
                                }
                                td {
                                    super::integration_type::Integration {
                                        integration_type: integration.integration_type
                                    }
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            href: "#",
                                            drawer_trigger: format!("show-integration-details-{}", integration.id),
                                            "Show Details"
                                        }
                                        DropDownLink {
                                            href: "#",
                                            drawer_trigger: format!("edit-integration-form-{}", integration.id),
                                            "Edit"
                                        }
                                        DropDownLink {
                                            drawer_trigger: format!("delete-trigger-{}-{}",
                                            integration.id, team_id),
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
