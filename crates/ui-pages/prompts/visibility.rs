#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Visibility;
use dioxus::prelude::*;

#[inline_props]
pub fn VisLabel(cx: Scope, visibility: Visibility) -> Element {
    match visibility {
        Visibility::Company => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Company"
            }
        )),
        Visibility::Private => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Private"
            }
        )),
        Visibility::Team => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Team"
            }
        )),
    }
}
