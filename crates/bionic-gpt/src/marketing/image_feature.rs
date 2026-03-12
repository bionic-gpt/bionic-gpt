use dioxus::prelude::*;

#[component]
pub fn ImageFeature(title: String, sub_title: String, image: String) -> Element {
    rsx! {
        section {
            class: "",
            h1 {
                class: "text-5xl font-bold text-center",
                "{title}"
            }
            h2 {
                class: "text-center mt-8",
                "{sub_title}"
            }
            img {
                src: "{image}",
                alt: "Product screenshot",
                class: "mt-8"
            }
        }
    }
}
