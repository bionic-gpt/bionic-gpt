#![allow(non_snake_case)]
use daisy_rsx::*;
use db;
use dioxus::prelude::*;

#[component]
pub fn RateTable(rate_limits: Vec<db::RateLimit>) -> Element {
    rsx!(
        Box {
            class: "has-data-table mt-6",
            BoxHeader {
                title: "Limits"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Role Name or User" }
                        th { "Model" }
                        th { "Limit" }
                        th {
                            class: "text-right",
                            "Action"
                        }
                    }
                    tbody {
                        tr {
                            td {
                                "Default"
                            }
                            td {
                                "All"
                            }
                            td {
                                "400 Tokens per hour"
                            }
                            td {
                                class: "text-right",
                                "..."
                            }
                        }
                        tr {
                            td {
                                "ian.purton@gmail.com"
                            }
                            td {
                                "All"
                            }
                            td {
                                "100 Tokens per hour"
                            }
                            td {
                                class: "text-right",
                                "..."
                            }
                        }
                        tr {
                            td {
                                "Power User"
                            }
                            td {
                                "LLama3"
                            }
                            td {
                                "1000 Tokens per hour"
                            }
                            td {
                                class: "text-right",
                                "..."
                            }
                        }
                    }
                }
            }
        }
    )
}
