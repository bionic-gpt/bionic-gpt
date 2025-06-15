#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Role as DBRole;
use dioxus::prelude::*;

#[component]
pub fn Role(role: DBRole) -> Element {
    match role {
        DBRole::SystemAdministrator => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Accent,
                "System Administrator"
            }
        ),
        DBRole::TeamManager => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Neutral,
                "Team Manager"
            }
        ),
        DBRole::Collaborator => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Neutral,
                "Collaborator"
            }
        ),
    }
}
