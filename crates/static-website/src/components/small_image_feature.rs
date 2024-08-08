use dioxus::prelude::*;

#[component]
pub fn SmallImageFeature(title: String, sub_title: String, text: String, image: String) -> Element {
    rsx! {
        section { class: "lg:max-w-5xl py-24 sm:py-32",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto max-w-2xl lg:text-center mb-6",
                    h2 { class: "badge badge-outline", "{title}" }
                    p { class: "mt-2 text-3xl font-bold tracking-tight sm:text-4xl text-primary",
                        "{sub_title}"
                    }
                    p { class: "mt-6 text-lg leading-8", "{text}" }
                }
                img {
                    alt: "Product screenshot",
                    src: "{image}",
                    class: "rounded-xl ring-1 ring-gray-400/10 lg:max-w-2xl"
                }
            }
            }
    }
}
