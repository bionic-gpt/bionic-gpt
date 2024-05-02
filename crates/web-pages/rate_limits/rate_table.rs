#![allow(non_snake_case)]
use daisy_rsx::*;
use db;
use dioxus::prelude::*;

#[component]
pub fn RateTable(rate_limits: Vec<db::RateLimit>, team_id: i32) -> Element {
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
                                    "{limit.limits_role.unwrap_or_default()}"

                                    "{limit.user_email.unwrap_or_default()}"
                                }
                                td {
                                    "{limit.model_name}"
                                }
                                td {
                                    "{limit.tokens_per_hour}"
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            drawer_trigger: format!("delete-trigger-{}-{}",
                                            limit.id, team_id),
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
