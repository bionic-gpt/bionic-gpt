#![allow(non_snake_case)]
use daisy_rsx::*;
use db::TopUser;
use dioxus::prelude::*;

#[component]
pub fn TopUserTable(top_users: Vec<TopUser>) -> Element {
    rsx!(
        Box {
            class: "has-data-table mt-4",
            BoxHeader {
                title: "Top 10 Users in the last 24 hours (by Characters Sent)"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Email" }
                        th { "Model Name" }
                        th { "Access Type" }
                        th {
                            class: "text-right",
                            "Characters Sent" }
                    }
                    tbody {

                        for user in top_users {
                            tr {
                                td {
                                    strong {
                                        "{user.email}"
                                    }
                                }
                                td {
                                    "{user.model_name}"
                                }
                                td {
                                    Label {
                                        class: "mr-2",
                                        label_role: LabelRole::Neutral,
                                        "User Interface"
                                    }
                                }
                                td {
                                    class: "text-right",
                                    code {
                                        "{user.total_tokens_sent}"
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
