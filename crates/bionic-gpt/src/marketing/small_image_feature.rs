use crate::marketing::product_showcase::ProductImage;
use dioxus::prelude::*;

#[component]
pub fn SmallImageFeature(
    title: String,
    sub_title: String,
    text: String,
    image: String,
    flip: bool,
    class: Option<String>,
) -> Element {
    let flip = if flip { "flex-row-reverse" } else { "flex-row" };
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: "{class} md:flex {flip} gap-8",
            div {
                class: "flex-1",
                h2 {
                    class: "badge badge-outline",
                    "{title}" }
                p {
                    class: "mt-8 text-3xl tracking-tight sm:text-4xl font-display",
                    "{sub_title}"
                }
                p {
                    class: "mt-6 text-lg leading-8",
                    "{text}"
                }
            }
            div {
                class: "flex-1",
                ProductImage {
                    image,
                    alt: "Product screenshot".to_string(),
                    framed: false,
                    class: Some("w-full".to_string()),
                    width: Some("728".to_string()),
                    height: Some("610".to_string()),
                    eager: false,
                }
            }
        }
    }
}
