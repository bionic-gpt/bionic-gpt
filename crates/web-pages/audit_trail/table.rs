#![allow(non_snake_case)]
use daisy_rsx::*;
use db::AuditTrail;
use dioxus::prelude::*;

#[component]
pub fn AuditTable(audits: Vec<AuditTrail>) -> Element {
    rsx!(
        Card {
            class: "has-data-table",
            CardHeader {
                title: "Audit Trail"
            }
            CardBody {
                table {
                    class: "table",
                    thead {
                        th { "When" }
                        th { "User" }
                        th {
                            class: "max-sm:hidden",
                            "Access Type"
                        }
                        th {
                            class: "text-right",
                            "Action"
                        }
                    }
                    tbody {
                        for audit in audits {
                            tr {
                                td {
                                    RelativeTime {
                                        format: RelativeTimeFormat::Relative,
                                        datetime: &audit.created_at
                                    }
                                }
                                td {
                                    "{audit.email}"
                                }
                                td {
                                    class: "max-sm:hidden",
                                    Badge {
                                        class: "mr-2",
                                        badge_color: BadgeColor::Neutral,
                                        {super::access_type_to_string(audit.access_type)}
                                    }
                                }
                                td {
                                    class: "text-right",
                                    Badge {
                                        badge_color: BadgeColor::Neutral,
                                        {super::audit_action_to_string(audit.action)}
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
