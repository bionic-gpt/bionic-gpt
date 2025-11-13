use dioxus::prelude::*;

pub const EXTRA_FOOTER_TITLE: &str =
    "The all-in-one agentic AI platform for regulated teams—secure, open, and extensible end to end.";

#[component]
pub fn ExtraFooter(title: String, image: String, cta: String, cta_url: String) -> Element {
    rsx! {
        section {
            class: "py-16 px-6 mt-24 w-full bg-secondary-content mb-0",
            div {
                class: "mx-auto flex max-w-6xl flex-col items-center gap-6 px-6 text-center",
                h2 {
                    class: "text-3xl font-bold max-w-3xl mx-auto leading-tight",
                    "{title}"
                }
                img {
                    class: "w-full max-w-3xl",
                    alt: "Product Screenshot",
                    src: "{image}"
                }
                div {
                    class: "flex flex-col gap-4 sm:flex-row sm:justify-center",
                    a {
                        href: "{cta_url}",
                        class: "btn btn-primary",
                        "{cta}"
                    }
                }
            }
        }
    }
}
