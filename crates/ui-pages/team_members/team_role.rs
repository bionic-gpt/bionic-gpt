#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role;
use dioxus::prelude::*;

#[component]
pub fn Role(role: Role) -> Element {
    match role {
        Role::SystemAdministrator => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "System Administrator"
            }
        ),
        Role::TeamManager => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Team Manager"
            }
        ),
        Role::Collaborator => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Neutral,
                "Collaborator"
            }
        ),
    }
}
