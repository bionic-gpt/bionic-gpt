#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn RateTable() -> Element {
    rsx!(
        Box {
            class: "has-data-table mt-6",
            BoxHeader {
                title: "Models"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Name" }
                        th { "Base URL" }
                        th { "Model Type" }
                        th { "Parameters" }
                        th { "Context Length" }
                        th {
                            class: "text-right",
                            "Action"
                        }
                    }
                    tbody {
                    }
                }
            }
        }
    )
}
