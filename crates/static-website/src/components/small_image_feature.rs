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
            class: "{class} lg:max-w-5xl md:flex {flip} gap-8",
            div {
                class: "flex-1",
                h2 {
                    class: "badge badge-outline",
                    "{title}" }
                p {
                    class: "mt-8 text-3xl tracking-tight sm:text-4xl text-primary",
                    "{sub_title}"
                }
                p {
                    class: "mt-6 text-lg leading-8",
                    "{text}"
                }
            }
            div {
                class: "flex-1",
                img {
                    loading: "lazy",
                    width: "728",
                    height: "610",
                    alt: "Product screenshot",
                    src: "{image}",
                }
            }
        }
    }
}
