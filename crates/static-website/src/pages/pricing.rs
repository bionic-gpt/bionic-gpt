use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn pricing() -> String {
    let page = rsx! {
        Layout {
            title: "Pricing",
            description: "Bionic Pricing",
            mobile_menu: None,
            section: Section::Pricing,
            div {
                div {
                    class: "p-5 mt-24 mx-auto max-w-7xl px-6 lg:px-8",
                    div {
                        class: "mx-auto max-w-2xl sm:text-center",
                        h2 {
                            class: "text-3xl font-bold tracking-tight sm:text-4xl", "Pricing" }
                        p {
                            class: "mt-6 text-lg leading-8",
                            "Bionic is the all-in-one platform for agentic AI—assistants, automations, APIs, and observability in a single stack.\n          Pick the deployment option that fits your governance model and let us help you wire it into Operations, Compliance, and Security."
                        }
                    }
                }
            }
            div {
                class: "mx-auto mt-12 mb-12 lg:flex lg:flex-row justify-center gap-4 lg:max-w-5xl w-full p-5",
                div {
                    class: "card card-border lg:w-1/2",
                    div {
                        class: "card-body flex flex-col justify-between list-tick",
                        div {
                            class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Community Edition" }
                            span { class: "badge badge-primary badge-outline", "Self Hosted" }
                            p {
                                "Deploy every piece of the Bionic platform in your own infrastructure.\n            Ideal for teams that want total control over data and workloads."
                            }
                            h4 { class: "font-extrabold", "Included Platform" }
                            ul {
                                li { "Assistants, automations, pipelines, and the full AI console." }
                                li { "Complete Agentic RAG toolkit with chunking, embeddings, and search." }
                                li { "APIs compatible with OpenAI for existing tooling." }
                            }
                            h4 { class: "font-extrabold", "Operate It Yourself" }
                            ul {
                                li { "Bring your own models, storage, and observability." }
                                li { "Unlimited namespaces—add projects whenever you need." }
                                li { "After the second user we show a gentle reminder to upgrade." }
                            }
                        }
                        div {
                            class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Free" }
                            span { class: "text-sm opacity-70", "Open source, permissive licence." }
                            a {
                                href: crate::routes::architect_course::Index {}.to_string(),
                                class: "btn btn-secondary btn-outline",
                                "\n            Deploy It Yourself\n          "
                            }
                        }
                    }
                }
                div {
                    class: "card card-border lg:w-1/2",
                    div {
                        class: "card-body flex flex-col justify-between list-tick",
                        div {
                            class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Production" }
                            span { class: "badge badge-primary badge-outline", "$699 / namespace / year" }
                            p {
                                "Roll out a branded agentic AI platform for your company with predictable pricing.\n            Perfect for teams that need scale, governance, and support."
                            }
                            h4 { class: "font-extrabold", "Scale Confidently" }
                            ul {
                                li { "Up to 1,000 users per namespace with SSO-ready auth." }
                                li { "High availability hosting and proactive monitoring." }
                                li { "Usage dashboards, rate limits, and API governance." }
                            }
                            h4 { class: "font-extrabold", "Make It Yours" }
                            ul {
                                li { "Whitelabel the UI with your logo and brand colors." }
                                li { "Custom domains plus private data connectors." }
                                li { "Access to roadmap previews and priority support." }
                            }
                        }
                        div {
                            class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "$699 / namespace / year" }
                            span { class: "text-sm opacity-70", "Includes onboarding and success check-ins." }
                            a {
                                href: crate::routes::marketing::Contact {}.to_string(),
                                class: "btn btn-secondary btn-outline",
                                "\n            Talk To Sales\n          "
                            }
                        }
                    }
                }
            }
            div {
                class: "mx-auto mb-16 lg:max-w-5xl w-full p-5",
                div {
                    class: "card card-border w-full",
                    div {
                        class: "card-body flex flex-col gap-4",
                        h3 { class: "card-title", "Consultancy & Expert Services" }
                        p {
                            "Need more than a subscription? Our engineering team ships product changes, installs Bionic in complex environments, and guides your GenAI roadmap."
                        }
                        ul {
                            class: "list-disc ml-6 space-y-2",
                            li { "Feature development or deep integrations with your existing systems." }
                            li { "Hands-on installation help across Kubernetes, bare metal, or air-gapped networks." }
                            li { "Advisory sessions on responsible AI, guardrails, and deployment playbooks." }
                        }
                        p { "Tell us what you are building and we will tailor an engagement around your goals." }
                        a {
                            href: crate::routes::marketing::Contact {}.to_string(),
                            class: "btn btn-secondary btn-outline",
                            "Book A Call"
                        }
                    }
                }
            }
            ExtraFooter {
                title: EXTRA_FOOTER_TITLE.to_string(),
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
            Footer {
                margin_top: "mt-0"
            }
        }
    };

    crate::render(page)
}
