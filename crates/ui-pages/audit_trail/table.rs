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
                                    Label {
                                        class: "mr-2",
                                        label_role: LabelRole::Neutral,
                                        super::access_type_to_string(audit.access_type)
                                    }
                                }
                                td {
                                    Label {
                                        class: "mr-2",
                                        label_role: LabelRole::Neutral,
                                        super::audit_action_to_string(audit.action)
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
