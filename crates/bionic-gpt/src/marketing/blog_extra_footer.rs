use dioxus::prelude::*;

pub fn blog_extra_footer() -> Element {
    rsx! {
        section {
            class: "blog-sub-footer mt-16 border-y border-base-300 bg-base-200 px-6 py-12 md:py-16",
            div {
                class: "mx-auto grid max-w-5xl items-center gap-8 md:grid-cols-2",
                div {
                    h2 {
                        class: "text-3xl font-bold leading-tight md:text-4xl",
                        "See what your users could solve themselves"
                    }
                    p {
                        class: "mt-4 text-lg opacity-80",
                        "Try the examples from this article in Bionic’s Zero to Agentic AI Hero course."
                    }
                    div {
                        class: "mt-6 flex flex-wrap gap-3",
                        a {
                            class: "btn btn-primary",
                            href: "/architect-course/",
                            "Take the course →"
                        }
                        a {
                            class: "btn btn-ghost",
                            href: "/",
                            "Explore Bionic →"
                        }
                    }
                }
                img {
                    class: "mx-auto w-full max-w-sm rounded-box shadow-md",
                    src: "/landing-page/bionic-console.png",
                    alt: "Bionic workspace"
                }
            }
        }
    }
}
