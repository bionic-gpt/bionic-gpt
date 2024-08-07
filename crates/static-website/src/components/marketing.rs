use std::fs::{self, File};
use std::io::Write;

use super::image_hero::ImageHero;
use crate::components::extra_footer::ExtraFooter;
use crate::components::image_feature::ImageFeature;
use crate::components::partners::Partners;
use crate::components::quad_feature::QuadFeature;
use crate::components::small_image_feature::SmallImageFeature;
use crate::layouts::layout::Layout;
use crate::routes::marketing::Index;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use dioxus::prelude::*;

pub fn routes() -> Router {
    Router::new().typed_get(index)
}

pub async fn generate() {
    let html = crate::render(HomePage).await;

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(Pricing).await;

    fs::create_dir_all("dist/pricing").expect("Couyldn't create folder");
    let mut file = File::create("dist/pricing/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(ContactPage).await;

    fs::create_dir_all("dist/contact").expect("Couyldn't create folder");
    let mut file = File::create("dist/contact/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub async fn index(Index {}: Index) -> Html<String> {
    let html = crate::render(HomePage).await;

    Html(html)
}

#[component]
pub fn Pricing() -> Element {
    rsx! {
        Layout {
            title: "Pricing",
            div {
                div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                    div { class: "mx-auto max-w-2xl sm:text-center",
                        h2 { class: "text-3xl font-bold tracking-tight sm:text-4xl", "Pricing" }
                        p { class: "mt-6 text-lg leading-8",
                            "\n          Bionic works best when it's integrated with your systems.\n          We offer packages to integrate Bionic with your Operations, Compliance and Security. \n        "
                        }
                    }
                }
            }
            div { class: "mx-auto lg:flex lg:flex-row justify-center gap-4 lg:max-w-5xl w-full p-5",
                div { class: "card card-bordered lg:w-1/3",
                    div { class: "card-body flex flex-col justify-between list-tick",
                        div { class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Cloud" }
                            span { class: "badge badge-primary badge-outline", "Free" }
                            p { "Ideal for evaluating Bionic and testing RAG use cases." }
                            h4 { class: "font-extrabold", "Generative AI" }
                            ul {
                                li { "Code and Text Generation" }
                                li { "Role based access control" }
                                li { "Chat History" }
                            }
                            h4 { class: "font-extrabold", "Retrieval Augmented Generation" }
                            ul {
                                li { "All major document formats processed." }
                                li { "Automated chunking and vector generation." }
                            }
                            h4 { class: "font-extrabold", "Open AI Compatible API" }
                            ul {
                                li { "Track and monitor API usage." }
                                li { "Share models between projects." }
                                li { "Key creation and revocation." }
                            }
                            h4 { class: "font-extrabold", "Document Pipelines" }
                            ul {
                                li { "Batch or stream document processing." }
                                li { "100s of sources." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Free" }
                            a {
                                href: "https://app.bionic-gpt.com",
                                class: "btn btn-secondary btn-outline",
                                "\n            Get Started\n          "
                            }
                        }
                    }
                }
                div { class: "card card-bordered lg:w-1/3",
                    div { class: "card-body flex flex-col justify-between list-tick",
                        div { class: "flex flex-col gap-3",
                            h3 { class: "card-title", "Encrypted Cloud" }
                            span { class: "badge badge-primary badge-outline", "Secure Compute" }
                            p { "Runs in our secure enclave with end to end encryption" }
                            h4 { class: "font-extrabold", "Everything from Cloud Edition and..." }
                            ul {
                                li { "Secure enclave running CPU resources." }
                                li { "Secure enclave running GPU resources." }
                                li { "Fully siloed and security hardenend." }
                            }
                            h4 { class: "font-extrabold", "Maximum Data Protection" }
                            ul {
                                li { "Built for running Generative AI on highly confidential data." }
                                li {
                                    "Hardware based secure compute thanks to our partnership with Nvidia and Google."
                                }
                                li { "Bring/Hold your own keys." }
                                li { "Provable supply chain with server attestation." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Custom Pricing" }
                            a {
                                href: "/contact",
                                class: "btn btn-secondary btn-outline",
                                "\n            Book a Call\n          "
                            }
                        }
                    }
                }
                div { class: "card card-bordered lg:w-1/3",
                    div { class: "card-body flex flex-col justify-between list-tick",
                        div { class: "flex flex-col gap-3",
                            h3 { class: "card-title", "On Premise / Private Cloud" }
                            span { class: "badge badge-primary badge-outline", "Enterprise Edition" }
                            p { "On Premise or Private Cloud" }
                            h4 { class: "font-extrabold",
                                "Everything from Cloud Edition and Encrypted Cloud..."
                            }
                            ul {
                                li { "Maximum privacy and security." }
                                li { "Support for running on bare metal." }
                                li { "Single Sign On." }
                            }
                            h4 { class: "font-extrabold", "Support" }
                            ul {
                                li { "Hardware recommendations." }
                                li { "Possibility of custom builds." }
                                li { "Dedicated customer success and engineering resources." }
                                li { "Custom Integrations." }
                                li { "Custom SLAs and support." }
                            }
                        }
                        div { class: "mt-5 flex flex-col gap-2",
                            hr {}
                            h3 { class: "font-extrabold", "Custom Pricing" }
                            a {
                                href: "/contact",
                                class: "btn btn-secondary btn-outline",
                                "\n            Book a Call\n          "
                            }
                        }
                    }
                }
            }
            ExtraFooter {
                title: "The secure open source Chat-GPT replacement
                that runs in a trusted execution environment for 
                maximum data security and compliance",
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
        }
    }
}

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            section {
                class: "mt-4 text-center mb-12",
                h1 {
                    class: "text-4xl font-extrabold mt-4",
                    "Our Team is Waiting to Hear From You"
                }
                h2 {
                    class: "text-2xl font-bold mt-4",
                    "Contact the Experts in Gen AI Deployments"
                }
                p {
                    class: "font-bold mt-4",
                    "Email founders (at) bionic-gpt.com"
                }
                p {
                    class: "mt-4 mb-4",
                    "Or Schedule a Meeting with Calendly"
                }
                a {
                    class: "btn btn-primary",
                    href: "https://calendly.com/bionicgpt",
                    "Schedule a Meeting"
                }
            }
            ExtraFooter {
                title: "The secure open source Chat-GPT replacement
                that runs in a trusted execution environment for 
                maximum data security and compliance",
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
        }
    }
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",

            div {
                class: "flex flex-col items-center",
                ImageHero {}
                Partners {}

                ImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    title1: "Regulatory Compliance.",
                    text1: "Run Generative AI and become compliant with GDPR, CCPA, PIPEDA, POPI, LGPD, HIPAA, PCI-DSS, and More",
                    title2: "Chat Console.",
                    text2: "A familiar chat console with text and code generation and the ability to select an assistant tuned on your data.",
                    title3: "Data Governance.",
                    text3: "By deploying Bionic close to your data you are able to benefit from Generative AI
        and still conform to data privacy and controls.",
                    image: "/landing-page/bionic-console.png",
                }

                SmallImageFeature {
                    title: "Confidential Computing",
                    sub_title: "Trusted Execution Environments",
                    text: "Don't spend time and resources re-inventing the wheel.
        We've developed an integrated solution using the best open source tools on the market
        to accelerate Gen AI adoption in your company.",
                    image: "/landing-page/confidential-compute.jpg"
                }

                ImageFeature {
                    title: "Retrieval Augmented Generation",
                    sub_title: "Build AI Assistants With Confidential Data",
                    text: "Teams manage their own datasets for use in RAG and fine tuning.",
                    title1: "Segmented Data.",
                    text1: "Teams manage their own data and can decide how best to share it.
                        Data is segregated at the database level.",
                    title2: "Self Manage Teams.",
                    text2: "There are no restrictions on the number of teams and teams are self managed.
                        Team administrators can add new users.",
                    title3: "Role Based Access Control",
                    text3: "Teams can manage the roles a user has from contributer to administrator. A central
                        system administrator role can manage the whole system.",
                    image: "/landing-page/assistants.png",
                }

                SmallImageFeature {
                    title: "Open Source",
                    sub_title: "Works with all Open Source LLMs",
                    text: "In most deployments the models are the bottleneck.
                        Bionic comes with a reverse proxy to monitor usage and apply limits to users
                        when needed",
                    image: "/landing-page/models.png"
                }

                QuadFeature {
                    title: "Cloud Native",
                    sub_title: "Private Cloud or Your Data Center",
                    text: "We fully support both options and can integrate with any provider",
                    title1: "Open Source Quantized Models.",
                    text1: "We integrate seemlessly with most open source AI models and out of the
        box we run against LLama 3 8B.",
                    title2: "Google, Amazon, Azure...",
                    text2: "If you choose to use a provider either from the public cloud or via a
        private cloud with have integrations with all the main suppliers.",
                    title3: "Multiple Models",
                    text3: "We can run against more than one model at a time allowing you to test use
        cases by easily switching between models",
                    title4: "Bare Metal",
                    text4: "Bionic has been deployed and tested on multiple bare metal Kubernetes clusters.
        You can run Bionic close to your private data for maximum control.",
                }

                SmallImageFeature {
                    title: "Support for PDF, Excel, Word, TXT, and more including OCR",
                    sub_title: "Integration with over 300 Data Sources",
                    text: "Our Data Pipeline API allows you to automate document uploads.",
                    image: "/landing-page/airbyte.png"
                }

                ImageFeature {
                    title: "Enterprise Grade Security",
                    sub_title: "Open Source under a Permissive Licence",
                    text: "Transport encryption, authentication, authorization, data segragation and more...",
                    title1: "SSO and Siem",
                    text1: "Our modular architecture allows us to adapt to your authentication and security needs.",
                    title2: "Support Contracts.",
                    text2: "Peace of mind knowing that the project maintainers are on call to help with your success.",
                    title3: "Consultancy",
                    text3: "We also can help with the full lifecycle of your Generative AI project. Trust the experts.",
                    image: "/landing-page/github.png",
                }

                SmallImageFeature {
                    title: "The easiest enterprise deployment you've ever seen",
                    sub_title: "Hundreds of installations around the world",
                    text: "Our high performance Rust solution is paired with Kubernetes for enterprise deployment stability.",
                    image: "/landing-page/bionic-startup-k9s.png"
                }
            }
        }
    }
}
