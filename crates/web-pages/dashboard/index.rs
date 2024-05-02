#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use daisy_rsx::*;
use db::authz::Rbac;
use db::TopUser;
use dioxus::prelude::*;

#[component]
pub fn Page(rbac: Rbac, team_id: i32, top_users: Vec<TopUser>) -> Element {
    rsx! {
        Layout {
            section_class: "normal h-[calc(100%-72px)]",
            selected_item: SideBar::Dashboard,
            team_id: team_id,
            rbac: rbac,
            title: "Dashboard",
            header: rsx! {
                h3 { "Dashboard (Trial)" }
            },
            div {
                class: "grid grid-cols-3 gap-4 h-full",
                Box {
                    BoxHeader {
                        title: "Card"
                    }
                    BoxBody {
                    }
                }
                Box {
                    BoxHeader {
                        title: "Card"
                    }
                    BoxBody {
                    }
                }
                Box {
                    BoxHeader {
                        title: "Card"
                    }
                    BoxBody {
                    }
                }
                Box {
                    class: "col-span-2 has-data-table",
                    BoxHeader {
                        title: "Guardrails Monitoring"
                    }
                    BoxBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "Email" }
                                th { "Source" }
                                th { "Type" }
                            }
                            tbody {
                                tr {
                                    td {
                                        "ian.purton@gmail.com"
                                    }
                                    td {
                                        Label {
                                            "Data Upload"
                                        }
                                    }
                                    td {
                                        Label {
                                            "PII Content"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    Box {
                        class: "grow",
                        BoxHeader {
                            title: "Card"
                        }
                        BoxBody {
                        }
                    }
                    Box {
                        class: "grow",
                        BoxHeader {
                            title: "Card"
                        }
                        BoxBody {
                        }
                    }
                }

                super::top_users_table::TopUserTable {
                    top_users: top_users
                }
            }
        }
    }
}
