use crate::components::benefits::Benefits;
use crate::components::features::{Feature, Features};
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navigation::Section;
use crate::components::testamonials::Testamonials;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

#[component]
pub fn PartnersPage() -> Element {
    let titles = &[
        "No Code Rag",
        "Team-based permissions",
        "Full Observability",
        "Rate limiting",
        "Military Grade Security",
        "Operations",
    ];

    let descriptions = &[
        "Including no-code RAG pipelines",
        "Data is siloed at the tema level",
        "Auto-assign tasks, send Slack messages, and much more...",
        "Audit-proof software built for critical financial...",
        "Craft beautiful, delightful experiences for both...",
        "Keep your company’s lights on with customizable...",
    ];

    let features: Vec<Feature> = titles
        .iter()
        .zip(descriptions.iter())
        .map(|(title, description)| Feature {
            title: title.to_string(),
            description: description.to_string(),
        })
        .collect();

    rsx! {
        Layout {
            title: "Partners",
            mobile_menu: None,
            description: "Partners",
            section: Section::Partners,
            div {
                class: "p-5 mt-24 flex flex-col items-center",

                Hero {
                    title: "Become a Bionic-GPT Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions"
                }

                Benefits {
                    title: "Partners",
                    subtitle: "Why Partner with Us?",
                    benefit1: "Revenue Growth",
                    benefit1_desc: "Earn from licensing new users, support, and upgrades,
                        while also providing AI consulting, training, and development services.",
                    benefit2: "In-Demand Solution",
                    benefit2_desc: "Our platform’s private, secure deployment model opens doors
                        to businesses prioritising data privacy and compliance.",
                    benefit3: "End-to-End Support",
                    benefit3_desc: "Get onboarding assistance and ongoing technical
                        support to ensure a seamless experience for you and your clients.",
                }

                Features {
                    title: "Bionic-GPT Features",
                    description: "As a bionicGPT partner, you can tap into a growing market of businesses seeking safe,
                        private, and powerful AI solutions.",
                    features
                }

                Testamonials {
                    text1: "The no-code RAG pipeline, combined with the team-based privacy model, has revolutionized
                        how we handle sensitive data and collaboration, making deployment secure and seamless.",
                    job1: "Data Governance Lead",
                    person1: "Emma Trident",
                    text2: "The new, clean, and intuitive interface has made adopting bionicGPT across teams effortless.
                        It’s a joy to use and has lowered the learning curve significantly.",
                    job2: "Digital Adoption Specialist",
                    person2: "Patrick O'leary",
                }

                section {
                    div {
                        class: "mt-10 flex flex-col items-center",
                        hr { class: "w-full mb-4" }
                        a {
                            href: "/contact",
                            class: "btn btn-secondary btn-outline",
                            "Book a Call"
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
