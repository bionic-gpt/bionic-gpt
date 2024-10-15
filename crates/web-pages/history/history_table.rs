#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;

#[component]
pub fn HistoryTable() -> Element {
    rsx!(
        Box {
            class: "has-data-table mb-6",
            BoxHeader {
                title: "Today"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Time" }
                        th { "Summary" }
                    }
                    tbody {
                        tr {
                            td {
                                strong {
                                    "5 minutes ago."
                                }
                            }
                            td {
                                a {
                                    href: "#",
                                    "Dioxus Drawer Component Update"
                                }
                            }
                        }
                    }
                }
            }
        }
        Box {
            class: "has-data-table mb-6",
            BoxHeader {
                title: "Yesterday"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Time" }
                        th { "Summary" }
                    }
                    tbody {
                    }
                }
            }
        }
        Box {
            class: "has-data-table mb-6",
            BoxHeader {
                title: "Last Week"
            }
            BoxBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Time" }
                        th { "Summary" }
                    }
                    tbody {
                    }
                }
            }
        }
    )
}
