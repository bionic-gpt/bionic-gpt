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
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "System Administrator"
            }
        ),
        DBRole::TeamManager => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Neutral,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Team Manager"
            }
        ),
        DBRole::Collaborator => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Neutral,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Collaborator"
            }
        ),
    }
}
