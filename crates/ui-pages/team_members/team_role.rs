#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role;
use dioxus::prelude::*;

#[inline_props]
pub fn Role(cx: Scope, role: Role) -> Element {
    match role {
        Role::SystemAdministrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "SystemAdministrator"
            }
        )),
        Role::TeamManager => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Team Manger"
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
