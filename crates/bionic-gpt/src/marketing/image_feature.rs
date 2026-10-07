use crate::marketing::product_showcase::ProductImage;
use dioxus::prelude::*;

#[component]
pub fn ImageFeature(title: String, sub_title: String, image: String) -> Element {
    rsx! {
        section {
            class: "",
            h1 {
                class: "text-center",
                "{title}"
            }
            h2 {
                class: "text-center mt-8",
                "{sub_title}"
            }
            ProductImage {
                image,
                alt: "Product screenshot".to_string(),
                framed: false,
                class: Some("mt-8".to_string()),
                width: None,
                height: None,
                eager: false,
            }
        }
    }
}
