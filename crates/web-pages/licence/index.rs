#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, version: String, remaining_days: i32) -> Element {
    rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Licence,
            team_id: team_id,
            rbac: rbac,
            title: "Licence",
            header: rsx! {
                h3 { "Your Licence" }
            },
            BlankSlate {
                heading: format!("You are running the Bionic Version {}", version),
                visual: bionic_logo_svg.name,
                description: format!("You have {} Days Trial Remaining", remaining_days),
                primary_action_drawer: Some(("Extend Trial or Licence Bionic".to_string(), "create-licence".to_string()))
            }
            Box {
                class: "has-data-table mt-8",
                BoxHeader {
                    title: "Features"
                }
                BoxBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Feature" }
                            th { "Unlicenced" }
                            th { "Trial" }
                            th { "Enterprise" }
                        }
                        tbody {
                            tr {
                                td {
                                    "AI Chat Console"
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Manage Models"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "AI Assistants & Datasets (R.A.G)"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Rate Limits"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Prometheus Metrics Endpoint"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "AI Observability and Compliance"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Virtual API Keys"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Data Integration Pipelines (Airbyte)"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                            tr {
                                td {
                                    "Support Agreement"
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: cross_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                                td {
                                    img {
                                        src: tick_svg.name,
                                        width: "16",
                                        height: "16"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Box {
                class: "has-data-table mt-8",
                BoxHeader {
                    title: "Custom Features"
                }
                BoxBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "Feature" }
                        }
                        tbody {
                            tr {
                                td {
                                    "Integration with Secure Enclaves"
                                }
                            }
                            tr {
                                td {
                                    "Custom Guardrails"
                                }
                            }
                            tr {
                                td {
                                    "Custom Theme"
                                }
                            }
                        }
                    }
                }
            }

            super::form::Form { }
        }
    }
}
