use dioxus::prelude::*;

#[component]
pub fn ProductHero(
    eyebrow: String,
    title: String,
    subtitle: String,
    claim: Option<String>,
    supporting: Option<String>,
) -> Element {
    rsx! {
        header { class: "max-w-3xl",
            p { class: "badge badge-outline", "{eyebrow}" }
            h1 { class: "site-page-title mt-5 text-4xl font-bold tracking-tight sm:text-5xl", "{title}" }
            p { class: "site-lead mt-5 text-lg leading-8 opacity-80", "{subtitle}" }
            if let Some(supporting) = supporting {
                p { class: "mt-4 max-w-2xl text-sm leading-6 opacity-70", "{supporting}" }
            }
            if let Some(claim) = claim {
                p { class: "mt-4 text-sm font-semibold opacity-70", "{claim}" }
            }
        }
    }
}
