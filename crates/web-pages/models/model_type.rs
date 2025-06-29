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
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Large Language Model"
            }
        ),
        ModelType::Embeddings => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Accent,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Embeddings Model"
            }
        ),
        ModelType::TextToSpeech => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Warning,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Text To Speech"
            }
        ),
        ModelType::Image => rsx!(
            Badge {
                class: "truncate",
                badge_color: BadgeColor::Neutral,
                badge_style: BadgeStyle::Outline,
                badge_size: BadgeSize::Sm,
                "Image Generation"
            }
        ),
    }
}
