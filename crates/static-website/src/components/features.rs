use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Feature {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[component]
pub fn Features(
    features: Vec<Feature>,
    title: String,
    description: String,
    class: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        section {
            class: format!("{class} body-font"),
            div {
                class: "mx-auto",
                div {
                    class: "mb-8 lg:mb-16",
                    h2 {
                        class: "mb-4 text-4xl tracking-tight font-display",
                        "{title}"
                    }
                    p {
                        class: "text-gray-500 sm:text-xl dark:text-gray-400",
                        "{description}"
                    }
                }
                div {
                    class: "space-y-8 md:grid md:grid-cols-2 lg:grid-cols-3 md:gap-12 md:space-y-0",
                    for feature in features {
                        div {
                            div {
                                class: "mb-4 w-10 h-10 lg:h-12 lg:w-12",
                                img {
                                    alt: "testimonial",
                                    src: "{feature.icon}"
                                }
                            }
                            h3 {
                                class: "mb-2 font-display text-xl font-bold",
                                "{feature.title}"
                            }
                            p {
                                "{feature.description}"
                            }
                        }
                    }
                }
            }
        }
    }
}
