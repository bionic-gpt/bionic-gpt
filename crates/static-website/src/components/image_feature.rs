use dioxus::prelude::*;

#[component]
pub fn ImageFeature(title: String, sub_title: String, image: String) -> Element {
    rsx! {
        section {
            class: "",
            h1 {
                "{title}"
            }
            h2 {
                "{sub_title}"
            }
            img {
                src: "{image}",
                alt: "Product screenshot",
                class: ""
            }
        }
    }
}
