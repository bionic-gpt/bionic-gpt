use dioxus::prelude::*;

#[component]
pub fn ImageHero() -> Element {
    rsx! {
        section {
            div {
                class: "text-center",
                div {
                    class: "max-w-md",
                    h1 {
                        class: "text-5xl font-bold",
                        "Generative AI. Private Data."
                    }
                    p {
                        class: "py-6",
                        "We use hardware based confidential computing to
                        run AI in a highly secure enclave for maximum 
                        protection of your data in the cloud or on premise"
                    }
                    div {
                        class: "flex gap-2 justify-center",
                        a {
                            class: "btn btn-primary",
                            href: "/cta1",
                            "Get started with Cloud Edition"
                        }
                        a {
                            class: "btn btn-secondary btn-outline",
                            href: "/cta2",
                            "Schedule a Meeting"
                        }
                    }
                }
            }
        }
    }
}
