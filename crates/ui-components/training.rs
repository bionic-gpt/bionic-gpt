use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
}

pub fn index(organisation_id: i32) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::Training,
                team_id: cx.props.organisation_id,
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

    crate::render(VirtualDom::new_with_props(app, Props { organisation_id }))
}
