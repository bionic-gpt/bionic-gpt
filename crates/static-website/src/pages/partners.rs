use crate::components::benefits::Benefits;
use crate::components::features::{Feature, Features};
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::navigation::Section;
use crate::components::testamonials::Testamonials;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn partners_page() -> String {
    let page = rsx! {
        Layout {
            title: "Partners",
            description: "Partners",
            section: Section::Partners,
            div {
                class: "lg:max-w-5xl mt-36 mx-auto grid gap-y-36",

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
                    description: "A comprehensive solution for all your AI needs.",
                    features: vec![
                        Feature {
                            title: String::from("Agentic Assistants"),
                            description: String::from("Connect assistants to your systems and your data."),
                            icon: "/features/systems.svg".to_string()
                        },
                        Feature {
                            title: String::from("Team-Based Permissions"),
                            description: String::from("Control data access and ensure security by allowing teams to manage permissions."),
                            icon: "/features/team.svg".to_string()
                        },
                        Feature {
                            title: String::from("Full Observability"),
                            description: String::from("Gain insights into usage and compliance with detailed dashboards and logs."),
                            icon: "/features/graph.svg".to_string()
                        },
                        Feature {
                            title: String::from("Cost Control"),
                            description: String::from("Set usage limits by user and team to manage costs effectively."),
                            icon: "/features/costs.svg".to_string()
                        },
                        Feature {
                            title: String::from("Advanced Encryption"),
                            description: String::from("Ensure data security with encryption at rest, in transit, and during runtime."),
                            icon: "/features/encryption.svg".to_string()
                        },
                        Feature {
                            title: String::from("Scalable Architecture"),
                            description: String::from("Built on Kubernetes for maximum scalability and reliability."),
                            icon: "/features/kubernetes.svg".to_string()
                        },
                    ]
                }

                Testamonials {
                    class: "mt-24",
                    text1: "The no-code RAG pipeline, combined with the team-based privacy model, has revolutionized
                        how we handle sensitive data and collaboration, making deployment secure and seamless.",
                    job1: "CEO GTEdge.ai",
                    img1: "/partners/tom-bendien.png",
                    person1: "Tom",
                    text2: "The new, clean, and intuitive interface has made adopting bionicGPT across teams effortless.
                        It’s a joy to use and has lowered the learning curve significantly. We're excited for the new possbilities.",
                    job2: "Digital Adoption Specialist",
                    person2: "Aisha",
                    img2: "/partners/aisha.png"
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
        Footer {
            extra_class: "mt-24"
        }
    };

    crate::render(page)
}
