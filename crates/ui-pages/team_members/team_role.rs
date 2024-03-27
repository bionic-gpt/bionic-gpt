#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role;
use dioxus::prelude::*;

#[component]
pub fn Role(cx: Scope, role: Role) -> Element {
    match role {
        Role::SystemAdministrator => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "System Administrator"
            }
        )),
        Role::TeamManager => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Team Manager"
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
