#![allow(non_snake_case)]
use db::DatasetConnection;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props {
    pub connection: DatasetConnection,
    pub datasets: String,
}

pub fn DatasetConnection(cx: Scope<Props>) -> Element {
    match cx.props.connection {
        DatasetConnection::All => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Done,
                label_contrast: LabelContrast::Primary,
                "Use All the Teams Datasets"
            }
        )),
        DatasetConnection::None => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_color: LabelColor::Attention,
                "Don't use any datasets"
            }
        )),
        DatasetConnection::Selected => cx.render(rsx!(Label {
            class: "mr-2",
            label_color: LabelColor::Attention,
            "Use selected Datasets"
        })),
    }
}
