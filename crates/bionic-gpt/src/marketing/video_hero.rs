use dioxus::prelude::*;

/// Requires the following to drastically improve google page insights performance.
/// <script type="module" src="https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"></script>

#[component]
pub fn VideoHero(
    title: String,
    subtitle: String,
    video_id: String,
    claim: String,
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
                    lite-youtube{
                        videoid: "{video_id}"
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
                    strong {
                        class: "hidden md:inline ml-4",
                        "{claim}"
                    }
                }
            }
        }
    }
}
