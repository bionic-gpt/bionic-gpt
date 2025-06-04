use dioxus::prelude::*;

use crate::routes::marketing;

#[component]
pub fn Hero(title: String, subtitle: String) -> Element {
    rsx! {
        section {
            class: "py-16 bg-gradient-to-r from-secondary/10 via-accent/10 to-primary/10",
            div {
                class: "flex justify-center text-center",
                div {
                    class: "max-w-lg px-8 py-4 rounded-box shadow-xl backdrop-blur-sm",
                    h1 {
                        class: "text-5xl font-extrabold mb-4 bg-gradient-to-r from-primary via-secondary to-accent bg-clip-text text-transparent",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary shadow",
                            href: marketing::Contact {}.to_string(),
                            "Book a Call"
                        }
                        a {
                            class: "btn btn-secondary btn-outline",
                            href: marketing::Pricing {}.to_string(),
                            "View Pricing"
                        }
                    }
                }
            }
        }
    }
}
