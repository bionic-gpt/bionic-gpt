#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn HistoryTable(buckets: Vec<super::index::HistoryBucket>) -> Element {
    rsx!(
        for bucket in buckets {
            if ! bucket.histories.is_empty() {
                Box {
                    class: "has-data-table mb-6",
                    BoxHeader {
                        title: "{bucket.name}"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Time" }
                                th { "Summary" }
                            }
                            tbody {
                                for history in bucket.histories {
                                    tr {
                                        td {
                                            strong {
                                                "{history.created_at}."
                                            }
                                        }
                                        td {
                                            a {
                                                href: "#",
                                                "{history.summary}"
                                            }
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
