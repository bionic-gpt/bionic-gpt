use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(image: String, title: String, subtitle: String) -> Element {
    rsx! {
        section {
            class: "mt-24 md:flex lg:max-w-5xl gap-8 w-full",
            div {
                class: "flex-1",
                h1 {
                    class: "text-2xl font-bold",
                    "{title}"
                }
                p {
                    class: "py-6",
                    "{subtitle}"
                }
            }
            div {
                class: "flex-1",
                img {
                    width: "560",
                    height: "315",
                    loading: "lazy",
                    class: "w-full aspect-[4/3]",
                    alt: "Product screenshot",
                    src: "{image}",
                }
            }
        }
    }
}
