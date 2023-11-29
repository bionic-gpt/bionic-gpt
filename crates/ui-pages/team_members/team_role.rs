#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role;
use dioxus::prelude::*;

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
    }
}
