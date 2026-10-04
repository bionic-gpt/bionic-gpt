use crate::marketing::{
    customer_logos::Customers,
    faq_accordian::{Faq, FaqText},
    footer::Footer,
    security::Security,
    use_case_hero::UseCaseHero,
    use_case_river::UseCaseRiver,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn home_page() -> String {
    let course_url = crate::routes::architect_course::Index {}.to_string();

    let page = rsx! {
        Layout {
            title: "Bionic – Open Source Sovereign AI Platform",
            description: "Bionic is the open-source foundation for internal AI teams building private, on-premise and sovereign AI capabilities.",
            mobile_menu: None,
            section: Section::Home,

            UseCaseHero {}

            div {
                class: "px-4 md:px-0 w-full lg:max-w-5xl mt-16 md:mt-36 mx-auto grid gap-y-28",
                Customers {}

                UseCaseRiver {}

                section {
                    class: "grid gap-8",
                    div {
                        class: "max-w-3xl",
                        p { class: "badge badge-outline", "Production foundation" }
                        h2 {
                            class: "mt-5 text-3xl font-bold tracking-tight sm:text-4xl",
                            "Start from a production foundation"
                        }
                        p {
                            class: "mt-4 text-lg leading-8 opacity-80",
                            "Internal AI teams should not spend months assembling generic infrastructure before they can deliver the first useful workflow."
                        }
                    }
                    div {
                        class: "grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-4",
                        for item in [
                            "AI workspace",
                            "Model connectivity",
                            "RAG and knowledge",
                            "Tools and integrations",
                            "Agentic runtime",
                            "Sandboxing",
                            "Identity and permissions",
                            "Audit",
                            "Scheduling",
                            "Artifact generation",
                            "Deployment infrastructure",
                        ] {
                            div {
                                class: "rounded-lg border border-base-300 bg-base-100 p-4 text-sm font-semibold shadow-sm",
                                "{item}"
                            }
                        }
                    }
                }

                section {
                    class: "grid gap-8",
                    div {
                        class: "max-w-3xl",
                        h2 {
                            class: "text-3xl font-bold tracking-tight sm:text-4xl",
                            "Your team should build AI capabilities, not another AI platform"
                        }
                        p {
                            class: "mt-4 text-lg leading-8 opacity-80",
                            "Bionic does not replace your AI team. It gives them a production-ready starting point."
                        }
                    }
                    div {
                        class: "grid gap-6 md:grid-cols-2",
                        div {
                            class: "card card-border bg-base-100",
                            div {
                                class: "card-body list-tick",
                                h3 { class: "card-title", "Bionic provides" }
                                ul {
                                    class: "space-y-2",
                                    li { "workspace" }
                                    li { "runtime" }
                                    li { "model connectivity" }
                                    li { "identity" }
                                    li { "permissions" }
                                    li { "audit" }
                                    li { "deployment foundation" }
                                    li { "extension framework" }
                                }
                            }
                        }
                        div {
                            class: "card card-border bg-base-100",
                            div {
                                class: "card-body list-tick",
                                h3 { class: "card-title", "Your team builds" }
                                ul {
                                    class: "space-y-2",
                                    li { "internal integrations" }
                                    li { "company-specific workflows" }
                                    li { "domain skills" }
                                    li { "business use cases" }
                                    li { "internal governance rules" }
                                    li { "differentiated capabilities" }
                                }
                            }
                        }
                    }
                }

                section {
                    class: "rounded-2xl bg-base-200 p-6 md:p-10",
                    div {
                        class: "grid gap-6 md:grid-cols-[1fr_auto] md:items-center",
                        div {
                            h2 {
                                class: "text-3xl font-bold tracking-tight",
                                "Learn how to build sovereign agentic AI"
                            }
                            p {
                                class: "mt-4 text-lg leading-8 opacity-80",
                                "A practical course for AI leads, architects and engineers building internal AI platforms."
                            }
                        }
                        a {
                            class: "btn btn-primary",
                            href: course_url,
                            "Start the course"
                        }
                    }
                }

                Faq {
                    questions: vec![
                        FaqText {
                            question: String::from("Is Bionic open source?"),
                            answer: String::from("Yes. The Community edition is open source and self-hosted."),
                        },
                        FaqText {
                            question: String::from("How is Bionic different from ChatGPT Enterprise?"),
                            answer: String::from("If hosted AI is approved for your workloads, ChatGPT Enterprise may be the right choice. Bionic is designed for organisations that require customer-controlled deployment, private or local models, or custom internal integrations."),
                        },
                        FaqText {
                            question: String::from("Can Bionic run fully on-premise?"),
                            answer: String::from("Yes. Bionic supports customer-controlled, private-cloud, on-premise and air-gapped deployment models."),
                        },
                        FaqText {
                            question: String::from("Can we use our own models?"),
                            answer: String::from("Yes. Bionic is model-independent and can use approved hosted models, private inference endpoints or local models."),
                        },
                        FaqText {
                            question: String::from("Can our developers build integrations?"),
                            answer: String::from("Yes. Bionic is designed to be extended by internal teams."),
                        },
                        FaqText {
                            question: String::from("Why not build our own platform?"),
                            answer: String::from("You can. Bionic exists to remove the generic platform work so your team can focus on differentiated workflows and integrations."),
                        },
                        FaqText {
                            question: String::from("What does Enterprise add?"),
                            answer: String::from("Enterprise provides production support, SLAs, supported releases, security response, lifecycle guidance and specialist engineering assistance."),
                        },
                        FaqText {
                            question: String::from("What is the Deployment Accelerator?"),
                            answer: String::from("A fixed-scope engagement to help deploy Bionic, connect core systems and deliver the first validated production workflow."),
                        },
                    ]
                }

                Security {}
            }
            Footer {
                links: footer_links()
            }
        }
    };

    crate::render(page)
}
