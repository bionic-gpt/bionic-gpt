use dioxus::prelude::*;

#[component]
pub fn ImageFeature(
    title: String,
    sub_title: String,
    text: String,
    title1: String,
    text1: String,
    title2: String,
    text2: String,
    title3: String,
    text3: String,
    image: String,
) -> Element {
    rsx! {
        section { class: "lg:max-w-5xl overflow-hidden py-24 sm:py-32",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2",
                    div { class: "lg:pr-8 lg:pt-4",
                        div { class: "lg:max-w-lg",
                            h2 {
                                class: "badge badge-outline",
                                "{title}"
                            }
                            p {
                                class: "mt-2 text-3xl font-bold tracking-tight sm:text-4xl text-primary",
                                "{sub_title}"
                            }
                            p {
                                class: "mt-6 text-lg leading-8",
                                "{text}"
                            }
                            dl { class: "mt-10 max-w-xl space-y-8 text-base leading-7 lg:max-w-none",
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold",
                                        svg {
                                            "fill": "currentColor",
                                            "aria-hidden": "true",
                                            "viewBox": "0 0 20 20",
                                            class: "absolute left-1 top-1 h-5 w-5",
                                            path {
                                                "fill-rule": "evenodd",
                                                "clip-rule": "evenodd",
                                                "d": "M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm3.75-2.75a.75.75 0 001.5 0V9.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0l-3.25 3.5a.75.75 0 101.1 1.02l1.95-2.1v4.59z"
                                            }
                                        }
                                        "{title1}"
                                    }
                                    dd { class: "inline", "{text1}" }
                                }
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold",
                                        svg {
                                            "aria-hidden": "true",
                                            "viewBox": "0 0 20 20",
                                            "fill": "currentColor",
                                            class: "absolute left-1 top-1 h-5 w-5",
                                            path {
                                                "clip-rule": "evenodd",
                                                "fill-rule": "evenodd",
                                                "d": "M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z"
                                            }
                                        }
                                        "{title2}"
                                    }
                                    dd { class: "inline", "{text2}" }
                                }
                                div { class: "relative pl-9",
                                    dt { class: "inline font-semibold",
                                        svg {
                                            "aria-hidden": "true",
                                            "fill": "currentColor",
                                            "viewBox": "0 0 20 20",
                                            class: "absolute left-1 top-1 h-5 w-5",
                                            path { "d": "M4.632 3.533A2 2 0 016.577 2h6.846a2 2 0 011.945 1.533l1.976 8.234A3.489 3.489 0 0016 11.5H4c-.476 0-.93.095-1.344.267l1.976-8.234z" }
                                            path {
                                                "fill-rule": "evenodd",
                                                "d": "M4 13a2 2 0 100 4h12a2 2 0 100-4H4zm11.24 2a.75.75 0 01.75-.75H16a.75.75 0 01.75.75v.01a.75.75 0 01-.75.75h-.01a.75.75 0 01-.75-.75V15zm-2.25-.75a.75.75 0 00-.75.75v.01c0 .414.336.75.75.75H13a.75.75 0 00.75-.75V15a.75.75 0 00-.75-.75h-.01z",
                                                "clip-rule": "evenodd"
                                            }
                                        }
                                        "{title3}"
                                    }
                                    dd { class: "inline", "{text3}" }
                                }
                            }
                        }
                    }
                    img {
                        height: "1442",
                        src: "{image}",
                        alt: "Product screenshot",
                        width: "2432",
                        class: "w-3xl max-w-none rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-228 md:-ml-4 lg:-ml-0"
                    }
                }
            }
        }
    }
}
