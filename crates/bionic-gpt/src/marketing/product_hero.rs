use dioxus::prelude::*;

#[component]
pub fn ProductHero(
    eyebrow: String,
    title: String,
    subtitle: String,
    claim: Option<String>,
    supporting: Option<String>,
    primary_cta: String,
    primary_href: String,
    secondary_cta: Option<String>,
    secondary_href: Option<String>,
    image: String,
    image_alt: String,
) -> Element {
    rsx! {
        header { class: "mx-auto w-full max-w-6xl px-6 py-12 sm:py-16 lg:py-20",
            div { class: "mx-auto max-w-3xl text-center",
                p { class: "site-eyebrow text-primary", "{eyebrow}" }
                h1 { class: "mt-4", "{title}" }
                p { class: "mx-auto mt-6 max-w-2xl text-base-content/70 sm:text-lg sm:leading-8", "{subtitle}" }
            if let Some(supporting) = supporting {
                p { class: "mx-auto mt-4 max-w-2xl text-sm leading-6 text-base-content/70", "{supporting}" }
            }
            if let Some(claim) = claim {
                p { class: "mt-4 text-sm font-semibold text-primary", "{claim}" }
            }
                div { class: "mt-8 flex flex-wrap items-center justify-center gap-3",
                    a { class: "btn btn-primary", href: "{primary_href}", "{primary_cta}" }
                    if let (Some(label), Some(href)) = (secondary_cta, secondary_href) {
                        a { class: "btn btn-outline", href: "{href}", "{label}" }
                    }
                }
            }
            figure { class: "mx-auto mt-12 max-w-6xl rounded-2xl bg-base-200 p-3 sm:mt-16 sm:p-6 lg:p-8",
                img { class: "mx-auto block h-auto w-full rounded-xl", src: "{image}", alt: "{image_alt}", loading: "eager" }
            }
        }
    }
}
