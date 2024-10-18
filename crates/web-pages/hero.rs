#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Hero(heading: String, subheading: String) -> Element {
    rsx!(
        div {
            class: "hero",
            div {
                class: "hero-content text-center",
                div {
                    class: "max-w-md",
                    h1 {
                        class: "text-3xl font-bold",
                        "{heading}"
                    }
                    p {
                        class: "py-6",
                        "{subheading}"
                    }
                }
            }
        }
    )
}
