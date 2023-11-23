#![allow(non_snake_case)]
use db::queries::documents::Document;
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
fn Page(cx: Scope, document: Document) -> Element {
    cx.render(rsx!(
        turbo-frame {
            id: "status-{document.id}",
            if document.waiting > 0 {
                cx.render(rsx!(
                    Label {
                        "Processing ({document.waiting} remaining)"
                    }
                ))
            } else if document.batches == 0 {
                cx.render(rsx!(
                    Label {
                        "Queued"
                    }
                ))
            }  else if document.fail_count > 0 {
                cx.render(rsx!(
                    Label {
                        label_role: LabelRole::Success,
                        "Processed ({document.fail_count} failed)"
                    }
                ))
            } else {
                cx.render(rsx!(
                    Label {
                        label_role: LabelRole::Success,
                        "Processed"
                    }
                ))
            }
        }
    ))
}

pub fn status(document: Document) -> String {
    let mut vdom = VirtualDom::new_with_props(Page, PageProps { document });

    let _ = vdom.rebuild();
    dioxus_ssr::render(&vdom)
}
