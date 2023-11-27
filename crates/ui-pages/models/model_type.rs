#![allow(non_snake_case)]
use daisy_rsx::*;
use db::ModelType;
use dioxus::prelude::*;

#[inline_props]
pub fn Model(cx: Scope, model_type: ModelType) -> Element {
    match model_type {
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
