#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use integrations::IntegrationTool;

#[component]
pub fn IntegrationTable(integrations: Vec<IntegrationTool>, team_id: i32) -> Element {
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
                        for (index, integration) in integrations.iter().enumerate() {
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
                    }
                }
            }
        }
    )
}
