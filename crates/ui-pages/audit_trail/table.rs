#![allow(non_snake_case)]
use daisy_rsx::*;
use db::AuditTrail;
use dioxus::prelude::*;

#[inline_props]
pub fn AuditTable<'a>(cx: Scope, audits: &'a Vec<AuditTrail>) -> Element {
    cx.render(rsx!(
        Box {
            class: "has-data-table",
            BoxHeader {
                title: "Audit Trail"
            }
            BoxBody {
                table {
                    thead {
                        th { "When" }
                        th { "User" }
                        th { "Access Type" }
                        th { "Action" }
                    }
                    tbody {
                        audits.iter().map(|audit| rsx!(
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
                                    super::access_type::AuditAccessType {
                                        access_type: audit.access_type
                                    }
                                }
                                td {
                                    super::audit_action::AuditAction {
                                        audit_action: audit.action
                                    }
                                }
                            }
                        ))
                    }
                }
            }
        }
    ))
}
