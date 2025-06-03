use dioxus::prelude::*;

use crate::routes::marketing;

#[component]
pub fn VideoHero(title: String, subtitle: String, video: String, claim: String) -> Element {
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
                    iframe {
                        class: "w-full aspect-video",
                        src: "{video}",
                        title: "YouTube video player",
                        "frameborder": "0",
                        allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                        referrerpolicy: "strict-origin-when-cross-origin",
                        allowfullscreen: true,
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
