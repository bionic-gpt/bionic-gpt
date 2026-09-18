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
                        "Your sovereign AI agent for enterprise tasks"
                    }
                    p {
                        class: "mt-4 text-lg opacity-80",
                        "Bionic connects AI to your organisation’s knowledge and tools, so it can research, analyse, create and take action. Deploy on-premise, in your private cloud or air-gapped, with full control over your data and models."
                    }
                    div {
                        class: "mt-6 flex flex-wrap gap-3",
                        a {
                            class: "btn btn-primary",
                            href: "/docs/running-locally/docker-compose/",
                            "Deploy Bionic"
                        }
                        a {
                            class: "btn btn-ghost",
                            href: crate::routes::marketing::Contact {}.to_string(),
                            "Talk to us about the Accelerator"
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
