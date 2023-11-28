#![allow(non_snake_case)]
use daisy_rsx::*;
use db::AuditAccessType;
use dioxus::prelude::*;

#[inline_props]
pub fn AuditAccessType(cx: Scope, access_type: AuditAccessType) -> Element {
    match access_type {
        AuditAccessType::API => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "CLI"
            }
        )),
        AuditAccessType::UserInterface => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Web App"
            }
        )),
    }
}
