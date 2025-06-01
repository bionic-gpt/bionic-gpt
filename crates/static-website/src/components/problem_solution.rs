use dioxus::prelude::*;

#[component]
pub fn ProblemSolution(
    image: String,
    title: String,
    problem: String,
    solution: String,
    class: Option<String>,
) -> Element {
    rsx! {
        section {
            class: format!("md:flex lg:max-w-5xl gap-8 w-full {}", class.unwrap_or("".to_string())),
            div {
                class: "flex-1",
                h1 {
                    class: "text-primary sm:text-3xl text-2xl font-medium",
                    "{title}"
                }
                p {
                    class: "py-6",
                    "{problem}"
                }
                p {
                    class: "py-6",
                    "{solution}"
                }
            }
            div {
                class: "flex-1",
                img {
                    width: "560",
                    height: "315",
                    loading: "lazy",
                    class: "w-full aspect-4/3",
                    alt: "Product screenshot",
                    src: "{image}",
                }
            }
        }
    }
}
