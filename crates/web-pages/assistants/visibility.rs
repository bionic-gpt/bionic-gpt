#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn VisLabel(visibility: Visibility) -> Element {
    match visibility {
        Visibility::Company => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Error,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
        Visibility::Private => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Accent,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
        Visibility::Team => rsx!(
            Badge {
                class: "mr-2",
                badge_color: BadgeColor::Info,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
    }
}
