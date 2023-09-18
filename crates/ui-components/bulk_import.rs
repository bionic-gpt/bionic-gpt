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
                section_class: "normal",
                selected_item: SideBar::BulkImport,
                team_id: cx.props.organisation_id,
                title: "Document Pipelines",
                header: cx.render(rsx!(
                    h3 { "Document Pipelines" }
                )),
                BlankSlate {
                    heading: "This feature is not complete yet",
                    visual: empty_api_keys_svg.name,
                    description: "When it is you'll be able to upload documents to S3 compatible buckets"
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        ApiKeysProps { organisation_id },
    ))
}
