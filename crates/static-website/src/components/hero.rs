use dioxus::prelude::*;

use crate::routes::marketing;

#[component]
pub fn Hero(
    title: String,
    subtitle: String,
    cta_label: Option<String>,
    cta_href: Option<String>,
) -> Element {
    let cta_label = cta_label.unwrap_or_else(|| "Book a Call".to_string());
    let cta_href = cta_href.unwrap_or_else(|| marketing::Contact {}.to_string());

    rsx! {
        section {
            div {
                class: "flex justify-center text-center",
                div {
                    class: "max-w-lg",
                    h1 {
                        class: "text-5xl font-bold",
                        "{title}"
                    }
                    p {
                        class: "py-6",
                        "{subtitle}"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary",
                            href: "{cta_href}",
                            "{cta_label}"
                        }
                    }
                }
            }
        }
    }
}
