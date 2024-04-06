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
                label_role: LabelRole::Highlight,
                "Company"
            }
        ),
        Visibility::Private => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Private"
            }
        ),
        Visibility::Team => rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Team"
            }
        ),
    }
}
