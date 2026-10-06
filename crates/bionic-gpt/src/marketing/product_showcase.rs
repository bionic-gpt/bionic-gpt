use dioxus::prelude::*;

#[component]
pub fn ProductShowcase(image: String, alt: String) -> Element {
    rsx! {
        ProductImage { image, alt, framed: true, class: None, width: None, height: None, eager: true }
    }
}

#[component]
pub fn ProductImage(
    image: String,
    alt: String,
    framed: bool,
    class: Option<String>,
    width: Option<String>,
    height: Option<String>,
    eager: bool,
) -> Element {
    let class = class.unwrap_or_default();
    let loading = if eager { "eager" } else { "lazy" };
    rsx! {
        if framed {
            figure { class: "product-showcase {class} overflow-hidden rounded-box border border-base-300 bg-base-200/50 p-3 sm:p-5",
                img {
                    class: "block h-auto w-full rounded-box border border-base-300 bg-base-100",
                    src: "{image}",
                    alt: "{alt}",
                    width: width.as_deref(),
                    height: height.as_deref(),
                    loading: "{loading}",
                }
            }
        } else {
            img {
                class: "{class} block h-auto max-w-full",
                src: "{image}",
                alt: "{alt}",
                width: width.as_deref(),
                height: height.as_deref(),
                loading: "{loading}",
            }
        }
    }
}
