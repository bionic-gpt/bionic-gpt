use db::queries::documents::Document;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    document: Document,
}

pub fn status(document: Document) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx!(
            turbo-frame {
                id: "status-{cx.props.document.id}",
                if cx.props.document.waiting > 0 {
                    cx.render(rsx!(
                        Label {
                            "Processing ({cx.props.document.waiting} remaining)"
                        }
                    ))
                } else if cx.props.document.batches == 0 {
                    cx.render(rsx!(
                        Label {
                            "Queued"
                        }
                    ))
                }  else if cx.props.document.fail_count > 0 {
                    cx.render(rsx!(
                        Label {
                            label_role: LabelRole::Highlight,
                            "Processed ({cx.props.document.fail_count} failed)"
                        }
                    ))
                } else {
                    cx.render(rsx!(
                        Label {
                            label_role: LabelRole::Highlight,
                            "Processed"
                        }
                    ))
                }
            }
        ))
    }

    let mut vdom = VirtualDom::new_with_props(app, Props { document });

    let _ = vdom.rebuild();
    dioxus_ssr::render(&vdom)
}
