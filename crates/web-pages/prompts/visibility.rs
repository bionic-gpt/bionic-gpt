#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[component]
pub fn VisLabel(visibility: Visibility) -> Element {
    match visibility {
        Visibility::Company => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Danger,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
        Visibility::Private => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
        Visibility::Team => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Info,
                "{crate::visibility_to_string(visibility)}"
            }
        ),
    }
}
