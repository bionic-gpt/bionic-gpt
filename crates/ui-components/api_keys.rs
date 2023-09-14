use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::*;

struct ApiKeysProps {
    organisation_id: i32,
}

pub fn api_keys(organisation_id: i32) -> String {
    fn app(cx: Scope<ApiKeysProps>) -> Element {
        cx.render(rsx! {
            Layout {
                selected_item: SideBar::ApiKeys,
                team_id: cx.props.organisation_id,
                title: "API Keys",
                header: cx.render(rsx!(
                    h3 { "API Keys" }
                )),
                BlankSlate {
                    heading: "Looks like you don't have any API keys",
                    visual: empty_api_keys_svg.name,
                    description: "API Keys allow you to access our programming interface",
                    primary_action_drawer: ("New API Key", "create-api-key")
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        ApiKeysProps { organisation_id },
    ))
}
