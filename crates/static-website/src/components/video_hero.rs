use dioxus::prelude::*;

use crate::routes::marketing;

/// Requires the following to drastically improve google page insights performance.
/// <script type="module" src="https://cdn.jsdelivr.net/npm/@justinribeiro/lite-youtube@1/lite-youtube.min.js"></script>

#[component]
pub fn VideoHero(title: String, subtitle: String, video_id: String, claim: String) -> Element {
    rsx! {
        section {
            class: "py-16 md:py-24 bg-gradient-to-br from-base-200 to-base-100",
            div {
                class: "md:flex flex-row gap-8 items-center text-center md:text-left",
                div {
                    class: "flex-1",
                    div {
                        h1 {
                            class: "text-3xl md:text-5xl font-extrabold mb-4 bg-gradient-to-r from-primary via-secondary to-accent bg-clip-text text-transparent",
                            "{title}"
                        }
                    }
                }
                div {
                    class: "flex-1",
                    lite-youtube{
                        class: "rounded-box shadow-2xl",
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
                        class: "btn btn-primary btn-wide shadow",
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
