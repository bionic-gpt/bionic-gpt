#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use daisy_rsx::*;
use dioxus::prelude::*;

#[inline_props]
pub fn Page(cx: Scope, organisation_id: i32) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::Training,
            team_id: *organisation_id,
            title: "Model Training",
            header: cx.render(rsx!(
                h3 { "Model Training" }
            )),
            BlankSlate {
                heading: "This feature is not complete yet",
                visual: empty_api_keys_svg.name,
                description: "When it is you'll be able to fine tune models to your data"
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
