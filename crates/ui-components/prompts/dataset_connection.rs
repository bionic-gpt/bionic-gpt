#![allow(non_snake_case)]
use db::DatasetConnection;
use dioxus::prelude::*;
use primer_rsx::*;

#[inline_props]
pub fn DatasetConnection(cx: Scope, connection: DatasetConnection) -> Element {
    match connection {
        DatasetConnection::All => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Use All the Teams Datasets"
            }
        )),
        DatasetConnection::None => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Don't use any datasets"
            }
        )),
        DatasetConnection::Selected => cx.render(rsx!(Label {
            class: "mr-2",
            label_role: LabelRole::Highlight,
            "Use selected Datasets"
        })),
    }
}
