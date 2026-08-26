#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn VisLabel(visibility: Visibility) -> Element {
    let color = match visibility {
        Visibility::Company => BadgeColor::Error,
        Visibility::Private => BadgeColor::Accent,
        Visibility::Team => BadgeColor::Info,
    };

    rsx!(
        Badge {
            class: "mr-2",
            badge_color: color,
            badge_style: BadgeStyle::Outline,
            badge_size: BadgeSize::Sm,
            "{crate::visibility_to_string(visibility)}"
        }
    )
}
