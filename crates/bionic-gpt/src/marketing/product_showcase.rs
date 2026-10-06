use dioxus::prelude::*;

#[component]
pub fn ProductShowcase(image: String, alt: String) -> Element {
    rsx! {
        figure { class: "overflow-hidden rounded-xl border border-base-300 bg-base-200/50 p-3 sm:p-5",
            img {
                class: "block h-auto w-full rounded-lg border border-base-300 bg-base-100",
                src: "{image}",
                alt: "{alt}",
                loading: "eager",
            }
        }
    }
}
