use dioxus::prelude::*;

#[component]
pub fn ImageHero(
    title: String,
    subtitle: String,
    image: String,
    cta_label: Option<String>,
    cta_href: Option<String>,
    class: Option<String>,
) -> Element {
    let cta_label = cta_label.unwrap_or_else(|| "Book a Call".to_string());
    let class = class.unwrap_or_default();
    rsx! {
        section {
            class: class,
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
                    if let Some(cta_href) = cta_href {
                        a {
                            class: "btn btn-secondary",
                            href: cta_href,
                            "{cta_label}"
                        }
                    }
                }
            }
        }
    }
}
