#![allow(non_snake_case)]
use daisy_rsx::*;
use db::ModelType;
use dioxus::prelude::*;

#[component]
pub fn Model(model_type: ModelType) -> Element {
    match model_type {
        ModelType::LLM => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Info,
                "Large Language Model"
            }
        ),
        ModelType::Embeddings => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Accent,
                "Embeddings Model"
            }
        ),
        ModelType::TextToSpeech => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Warning,
                "Text To Speech"
            }
        ),
        ModelType::Image => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Neutral,
                "Image Generation"
            }
        ),
    }
}
