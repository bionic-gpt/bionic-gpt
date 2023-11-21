#![allow(non_snake_case)]
use db::ModelType;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq, Eq)]
pub struct Props<'a> {
    pub model_type: &'a ModelType,
}

pub fn Model<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    match cx.props.model_type {
        ModelType::LLM => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Info,
                "Large Language Model"
            }
        )),
        ModelType::Embeddings => cx.render(rsx!(
            Label {
                class: "mr-2",
                label_role: LabelRole::Highlight,
                "Embeddings Model"
            }
        )),
    }
}
