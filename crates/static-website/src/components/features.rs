use dioxus::prelude::*;

#[component]
pub fn GraphSvg() -> Element {
    rsx! {
        svg {
            fill: "currentColor",
            width: "50",
            height: "50",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                fill_rule: "evenodd",
                d: "M3 3a1 1 0 000 2v8a2 2 0 002 2h2.586l-1.293 1.293a1 1 0 101.414 1.414L10
                15.414l2.293 2.293a1 1 0 001.414-1.414L12.414 15H15a2 2 0 002-2V5a1 1 0 
                100-2H3zm11.707 4.707a1 1 0 00-1.414-1.414L10 9.586 8.707 8.293a1 1 0 
                00-1.414 0l-2 2a1 1 0 101.414 1.414L8 10.414l1.293 1.293a1 1 0 001.414 0l4-4z",
                clip_rule: "evenodd",
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Feature {
    pub title: String,
    pub description: String,
}

#[component]
pub fn Features(features: Vec<Feature>, title: String, description: String) -> Element {
    rsx! {
        section {
            class: "lg:max-w-5xl body-font mt-24",
            div {
                class: "py-8 px-4 mx-auto max-w-screen-xl sm:py-16 lg:px-6",
                div {
                    class: "max-w-screen-md mb-8 lg:mb-16",
                    h2 {
                        class: "mb-4 text-4xl tracking-tight font-extrabold text-primary",
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
                                class: "flex justify-center items-center mb-4 w-10 h-10 rounded-full bg-primary-100 lg:h-12 lg:w-12",
                                GraphSvg {}
                            }
                            h3 {
                                class: "mb-2 text-xl font-bold dark:text-white",
                                "{feature.title}"
                            }
                            p {
                                class: "text-gray-500 dark:text-gray-400",
                                "{feature.description}"
                            }
                        }
                    }
                }
            }
        }
    }
}
