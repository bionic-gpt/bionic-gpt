#![allow(non_snake_case)]
use daisy_rsx::*;
use db::TopUser;
use dioxus::prelude::*;

#[inline_props]
pub fn TopUserTable<'a>(cx: Scope, top_users: &'a Vec<TopUser>) -> Element {
    cx.render(rsx!(
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

                        top_users.iter().map(|user| {
                            cx.render(rsx!(
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
                            ))
                        })
                    }
                }
            }
        }
    ))
}
