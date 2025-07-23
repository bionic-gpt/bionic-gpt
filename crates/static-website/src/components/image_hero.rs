use dioxus::prelude::*;

use crate::routes::marketing;

#[component]
pub fn ImageHero(title: String, subtitle: String, image: String) -> Element {
    rsx! {
        section {
            div {
                class: "flex flex-col md:flex-row gap-8 text-center md:text-left",
                div {
                    class: "flex-1",
                    div {
                        h1 {
                            class: "font-display text-2xl md:text-6xl font-bold",
                            "{title}"
                        }
                    }
                }
                div {
                    class: "flex-1",
                    img {
                        src: "{image}"
                    }
                }
            }
            div {
                class: "text-center",
                p {
                    class: "py-6",
                    "{subtitle}"
                }
                div {
                    a {
                        class: "btn btn-secondary",
                        href: marketing::Contact {}.to_string(),
                        "Book a Call"
                    }
                }
            }
        }
    }
}
