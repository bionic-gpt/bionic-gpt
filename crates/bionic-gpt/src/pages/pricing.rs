use crate::marketing::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn pricing() -> String {
    let contact_url = crate::routes::marketing::Contact {}.to_string();

    let page = rsx! {
        Layout {
            title: "Pricing",
            description: "Open-source sovereign AI platform pricing for self-hosted, enterprise and deployment accelerator options.",
            mobile_menu: None,
            section: Section::Pricing,
            div {
                class: "p-5 mt-24 mx-auto max-w-7xl px-6 lg:px-8",
                div {
                    class: "mx-auto max-w-3xl text-center",
                    h1 {
                        class: "text-4xl font-bold tracking-tight sm:text-5xl",
                        "Sovereign AI starts with an open-source foundation"
                    }
                    p {
                        class: "mt-6 text-lg leading-8",
                        "Bionic is the open-source foundation for teams that need to deploy sovereign AI without rebuilding the entire platform themselves."
                    }
                    p {
                        class: "mt-4 text-base leading-7 opacity-80",
                        "Your team owns the models, infrastructure, integrations and use cases. Bionic provides the production foundation so you can focus on the capabilities that differentiate your organisation."
                    }
                }
            }

            section {
                class: "mx-auto mt-12 mb-16 grid max-w-7xl grid-cols-1 gap-6 p-5 lg:grid-cols-3",
                div {
                    class: "card card-border bg-base-100",
                    div {
                        class: "card-body flex flex-col justify-between gap-6 list-tick",
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "flex items-center justify-between gap-3",
                                h2 { class: "card-title", "Community" }
                                span { class: "badge badge-outline", "Open source" }
                            }
                            div {
                                p { class: "text-4xl font-extrabold", "Free" }
                                p {
                                    class: "mt-2 text-sm opacity-70",
                                    "An open-source foundation for teams that want to deploy and extend Bionic themselves."
                                }
                            }
                            ul {
                                class: "space-y-2",
                                li { "Self-hosted deployment" }
                                li { "Core AI workspace" }
                                li { "Support for private and local models" }
                                li { "Document search and knowledge features" }
                                li { "Agentic runtime and tools" }
                                li { "Integration and extension framework" }
                                li { "Community support" }
                                li { "Access to the open-source codebase" }
                            }
                        }
                        a {
                            href: "/docs/running-locally/docker-compose/",
                            class: "btn btn-secondary btn-outline w-full",
                            "Get Started"
                        }
                    }
                }

                div {
                    class: "card card-border border-primary bg-base-100 shadow-xl",
                    div {
                        class: "card-body flex flex-col justify-between gap-6 list-tick",
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "flex items-center justify-between gap-3",
                                h2 { class: "card-title", "Enterprise" }
                                span { class: "badge badge-primary", "Recommended" }
                            }
                            div {
                                p { class: "text-4xl font-extrabold", "Starting at €40,000" }
                                p { class: "text-sm font-semibold opacity-80", "per year" }
                                p {
                                    class: "mt-2 text-sm opacity-70",
                                    "Pricing depends on deployment scope, environments and support requirements."
                                }
                            }
                            p {
                                class: "text-sm leading-6",
                                "Production assurance for organisations running Bionic as critical internal infrastructure."
                            }
                            ul {
                                class: "space-y-2",
                                li { "Premium technical support" }
                                li { "Defined support SLAs" }
                                li { "Security updates and vulnerability response" }
                                li { "Supported production releases" }
                                li { "Long-term support versions" }
                                li { "Upgrade and migration guidance" }
                                li { "Architecture and deployment reviews" }
                                li { "High-availability deployment guidance" }
                                li { "Backup and disaster-recovery guidance" }
                                li { "Air-gapped deployment support" }
                                li { "Security and compliance documentation" }
                                li { "Direct access to the Bionic engineering team" }
                            }
                        }
                        a {
                            href: contact_url.clone(),
                            class: "btn btn-primary w-full",
                            "Talk to Us"
                        }
                    }
                }

                div {
                    class: "card card-border bg-base-100",
                    div {
                        class: "card-body flex flex-col justify-between gap-6 list-tick",
                        div {
                            class: "flex flex-col gap-4",
                            div {
                                class: "flex items-center justify-between gap-3",
                                h2 { class: "card-title", "Deployment Accelerator" }
                                span { class: "badge badge-secondary", "launch" }
                            }
                            div {
                                p { class: "text-4xl font-extrabold", "Starting at €50,000" }
                                p {
                                    class: "mt-2 text-sm opacity-70",
                                    "Deploy a production-ready sovereign AI foundation and first working use case without months of platform assembly."
                                }
                            }
                            ul {
                                class: "space-y-2",
                                li { "Production deployment of Bionic" }
                                li { "Customer cloud, private cloud or on-premise deployment" }
                                li { "SSO and identity integration" }
                                li { "One approved model provider or local model" }
                                li { "One document or knowledge source" }
                                li { "One internal business system or tool" }
                                li { "Logging, audit and access-control configuration" }
                                li { "Security architecture review" }
                                li { "Administrator and developer enablement" }
                                li { "One validated business workflow" }
                                li { "Handover documentation" }
                                li { "Defined post-launch support period" }
                            }
                            div {
                                class: "rounded-lg bg-base-200 p-4 text-sm font-semibold leading-6",
                                "Expected outcome: a working Bionic deployment, an enabled internal team and one production-ready AI workflow."
                            }
                        }
                        a {
                            href: contact_url.clone(),
                            class: "btn btn-secondary btn-outline w-full",
                            "Plan a Deployment"
                        }
                    }
                }
            }

            section {
                class: "mx-auto mb-16 max-w-7xl p-5",
                div {
                    class: "mb-6 max-w-3xl",
                    h2 { class: "text-3xl font-bold tracking-tight", "Compare offers" }
                    p {
                        class: "mt-3 opacity-80",
                        "Core platform capabilities remain open source. Enterprise and accelerator options add accountability, assurance and delivery support for production environments."
                    }
                }
                div {
                    class: "overflow-x-auto rounded-xl border border-base-300",
                    table {
                        class: "table table-zebra min-w-[760px]",
                        thead {
                            tr {
                                th { "Capability" }
                                th { "Community" }
                                th { "Enterprise" }
                                th { "Deployment Accelerator" }
                            }
                        }
                        tbody {
                            tr { th { "Open-source platform" } td { "Included" } td { "Included" } td { "Included" } }
                            tr { th { "Self-hosted deployment" } td { "Included" } td { "Included" } td { "Implemented" } }
                            tr { th { "Private and local models" } td { "Included" } td { "Supported" } td { "Configured" } }
                            tr { th { "Integration framework" } td { "Included" } td { "Supported" } td { "Configured" } }
                            tr { th { "Community support" } td { "Included" } td { "Included" } td { "Included" } }
                            tr { th { "Premium support" } td { "-" } td { "Included" } td { "Post-launch period" } }
                            tr { th { "Support SLA" } td { "-" } td { "Defined SLA" } td { "Defined period" } }
                            tr { th { "Production architecture review" } td { "-" } td { "Included" } td { "Included" } }
                            tr { th { "Security and upgrade guidance" } td { "-" } td { "Included" } td { "Included" } }
                            tr { th { "Deployment implementation" } td { "Self-service" } td { "Guided" } td { "Included" } }
                            tr { th { "SSO configuration" } td { "Self-service" } td { "Guided" } td { "Included" } }
                            tr { th { "Initial integrations" } td { "Self-service" } td { "Guided" } td { "Included" } }
                            tr { th { "First production workflow" } td { "Self-service" } td { "Guided" } td { "Included" } }
                            tr { th { "Team enablement" } td { "Community docs" } td { "Available" } td { "Included" } }
                        }
                    }
                }
            }

            section {
                class: "mx-auto mb-16 max-w-5xl p-5",
                div {
                    class: "card card-border bg-base-100",
                    div {
                        class: "card-body",
                        h2 { class: "text-3xl font-bold tracking-tight", "Build what differentiates your organisation" }
                        p {
                            class: "mt-4 text-lg leading-8",
                            "Your AI team should be building integrations, workflows and domain-specific capabilities—not another generic AI platform."
                        }
                        p {
                            class: "mt-3 leading-7 opacity-80",
                            "Bionic provides the workspace, runtime, model connectivity, identity, audit and deployment foundation. Your team remains in control and can extend the platform using its own tools, systems and engineering standards."
                        }
                    }
                }
            }

            section {
                class: "mx-auto mb-16 max-w-5xl p-5",
                div {
                    class: "mb-6",
                    h2 { class: "text-3xl font-bold tracking-tight", "FAQ" }
                }
                div {
                    class: "join join-vertical w-full",
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "Is Bionic still open source?" }
                        div {
                            class: "collapse-content",
                            p { "Yes. The Community edition remains open source and self-hosted. Enterprise customers pay for production assurance, support, lifecycle management and specialist engineering assistance." }
                        }
                    }
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "Do we have to use Bionic's models?" }
                        div {
                            class: "collapse-content",
                            p { "No. Customers can connect approved hosted models or deploy local models within their own infrastructure." }
                        }
                    }
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "Can Bionic run fully on-premise?" }
                        div {
                            class: "collapse-content",
                            p { "Yes. Bionic is designed for customer-controlled, private-cloud, on-premise and air-gapped environments." }
                        }
                    }
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "What is included in the Deployment Accelerator?" }
                        div {
                            class: "collapse-content",
                            p { "The accelerator covers production deployment, identity, model configuration, initial integrations, team enablement and delivery of the first validated workflow." }
                        }
                    }
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "Why pay for Enterprise if Bionic is open source?" }
                        div {
                            class: "collapse-content",
                            p { "Enterprise provides defined accountability for production systems, including support SLAs, security response, supported releases, architecture guidance and upgrade assistance." }
                        }
                    }
                    div {
                        class: "collapse join-item collapse-arrow border border-base-300",
                        input { r#type: "checkbox" }
                        div { class: "collapse-title text-lg font-semibold", "Can our developers build their own integrations?" }
                        div {
                            class: "collapse-content",
                            p { "Yes. That is a core part of the product strategy. Bionic provides the foundation while internal teams build organisation-specific integrations and workflows." }
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
                margin_top: "mt-0",
                links: footer_links()
            }
        }
    };

    crate::render(page)
}
