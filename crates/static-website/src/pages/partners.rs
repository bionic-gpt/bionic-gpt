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
                class: "lg:max-w-5xl p-5 mt-8 md:mt-24 mx-auto",

                Hero {
                    title: "Become a Bionic-GPT Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions"
                }

                Benefits {
                    class: "mt-24",
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
                    class: "mt-24",
                    title: "Bionic-GPT Features",
                    description: "As a bionicGPT partner, you can tap into a growing market of businesses seeking safe,
                        private, and powerful AI solutions.",
                    features: vec![
                        Feature {
                            title: String::from("No Code Rag"),
                            description: String::from("Including no-code RAG pipelines"),
                        },
                        Feature {
                            title: String::from("Team-based permissions"),
                            description: String::from("Data is siloed at the team level"),
                        },
                        Feature {
                            title: String::from("Full Observability"),
                            description: String::from("Auto-assign tasks, send Slack messages, and much more..."),
                        },
                        Feature {
                            title: String::from("Rate limiting"),
                            description: String::from("Audit-proof software built for critical financial..."),
                        },
                        Feature {
                            title: String::from("Military Grade Security"),
                            description: String::from("Craft beautiful, delightful experiences for both..."),
                        },
                        Feature {
                            title: String::from("Operations"),
                            description: String::from("Keep your company’s lights on with customizable..."),
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
                        It’s a joy to use and has lowered the learning curve significantly.",
                    job2: "Digital Adoption Specialist",
                    person2: "Patrick",
                    img2: "https://dummyimage.com/106x106"
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
