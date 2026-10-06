use dioxus::prelude::*;

#[component]
pub fn ProductHero(eyebrow: String, title: String, subtitle: String, claim: String) -> Element {
    rsx! {
        header { class: "max-w-3xl",
            p { class: "badge badge-outline", "{eyebrow}" }
            h1 { class: "mt-5 text-4xl font-bold tracking-tight sm:text-5xl", "{title}" }
            p { class: "mt-5 text-lg leading-8 opacity-80", "{subtitle}" }
            p { class: "mt-4 text-sm font-semibold opacity-70", "{claim}" }
        }
    }
}
