#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, version: String) -> Element {
    rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Licence,
            team_id: team_id,
            rbac: rbac,
            title: "Licence",
            header: rsx! {
                h3 { "Your Licence" }
            },
            BlankSlate {
                heading: format!("You are running the Community Edition of Bionic ({})", version),
                visual: bionic_logo_svg.name,
                description: "Add a licence to extend available resources and features",
                primary_action_drawer: Some(("Licence Bionic".to_string(), "create-licence".to_string()))
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
                            th { "Community Edition" }
                            th { "Edge Edition" }
                            th { "Enterprise" }
                        }
                        tbody {
                            tr {
                                td {
                                    "Input and Output Guardrails"
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
                                    "Batch Guardrails"
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
                            tr {
                                td {
                                    "Observability Metrics Endpoint"
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

            super::form::Form { }
        }
    }
}
