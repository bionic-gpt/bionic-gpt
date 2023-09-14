use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::*;

struct ApiKeysProps {
    organisation_id: i32,
}

pub fn bulk(organisation_id: i32) -> String {
    fn app(cx: Scope<ApiKeysProps>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::BulkImport,
                team_id: cx.props.organisation_id,
                title: "Bulk Import",
                header: cx.render(rsx!(
                    h3 { "Bulk Import" }
                )),
                BlankSlate {
                    heading: "You haven't setup any bulk imports yet",
                    visual: empty_api_keys_svg.name,
                    description: "API Keys allow you to access our programming interface",
                    primary_action_drawer: ("New Bulk Import", "create-api-key")
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        ApiKeysProps { organisation_id },
    ))
}
