use crate::app_layout::{Layout, SideBar};
use assets::files::*;
use dioxus::prelude::*;
use primer_rsx::*;

struct PipelineKeysProps {
    organisation_id: i32,
}

pub fn index(organisation_id: i32) -> String {
    fn app(cx: Scope<PipelineKeysProps>) -> Element {
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
                    heading: "Automate document upload with our bulk upload API",
                    visual: empty_api_keys_svg.name,
                    description: "The upload API connects your documents to datasets for processing by our pipeline"
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        PipelineKeysProps { organisation_id },
    ))
}
