#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn HistoryTable(buckets: Vec<super::index::HistoryBucket>, total_count: usize) -> Element {
    rsx!(
        if total_count == 0 {

        } else {
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
                                    th {
                                        "width": "200",
                                        "Time"
                                    }
                                    th { "Summary" }
                                }
                                tbody {
                                    for history in bucket.histories {
                                        tr {
                                            td {
                                                RelativeTime {
                                                    format: RelativeTimeFormat::Relative,
                                                    datetime: &history.created_at_iso
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
        }
    )
}
