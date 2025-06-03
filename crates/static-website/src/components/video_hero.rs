use dioxus::prelude::*;

use crate::routes::marketing;

/// Requires the following to drastically improve google page insights performance.
/// <script type="module" src="https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"></script>

#[component]
pub fn VideoHero(title: String, subtitle: String, video_id: String, claim: String) -> Element {
    rsx! {
        section {
            div {
                class: "md:flex flex-row gap-8 text-center md:text-left",
                div {
                    class: "flex-1",
                    div {
                        h1 {
                            class: "text-primary text-2xl md:text-5xl font-bold",
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
                    a {
                        class: "btn btn-secondary",
                        href: marketing::Contact {}.to_string(),
                        "Book a Call"
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
