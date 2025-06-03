use dioxus::prelude::*;

#[component]
pub fn ExtraFooter(title: String, image: String, cta: String, cta_url: String) -> Element {
    rsx! {
        section {
            class: "mt-24 flex flex-col items-center text-center p-4 bg-secondary-content",
            h2 {
                class: "mt-4 mb-4 max-w-lg text-2xl font-bold",
                "{title}"
            }
            img {
                class: "lg:max-w-md",
                alt: "Product Screenshot",
                src: "{image}"
            }
            div {
                class: "mt-4 flex flex-col space-y-4 sm:flex-row sm:space-y-0 sm:space-x-4",
                a {
                    href: "{cta_url}",
                    class: "btn btn-primary",
                    "{cta}"
                }
            }
        }
    }
}
