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
                title: "Rate Limits"
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
                        for limit in rate_limits {
                            tr {
                                td {
                                    "{limit.limits_role}"
                                }
                                td {
                                    "{limit.user_email}"
                                }
                                td {
                                    "{limit.model_name}"
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
        }
    )
}
