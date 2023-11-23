#![allow(non_snake_case)]
use db::Role;
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn Role(cx: Scope, role: Role) -> Element {
    match role {
        Role::Administrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Administrator"
            }
        )),
        Role::Collaborator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Collaborator"
            }
        )),
        Role::SystemAdministrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Info,
                "System Administrator"
            }
        )),
    }
}
