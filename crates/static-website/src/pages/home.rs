use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::faq_accordian::{Faq, FaqText};
use crate::components::features::BionicFeatures;
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::components::security::Security;
use crate::components::small_image_feature::SmallImageFeature;
use crate::components::testamonials::Testamonial1;
use crate::components::video_hero::VideoHero;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn home_page() -> String {
    let page = rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "px-4 md:px-0 w-full lg:max-w-5xl mt-16 md:mt-36 mx-auto grid gap-y-36",
                VideoHero {
                    video_id: "slRiOOM17tM",
                    title: "The all-in-one platform for private and secure AI",
                    subtitle: "Deploy anywhere — on-prem, private cloud, or fully managed by us",
                    claim: "Join hundreds of teams already powering AI with Bionic"
                }

                Customers {
                }

                SmallImageFeature {
                    title: "Agentic AI",
                    sub_title: "Create AI Agents in seconds",
                    text: "Use default agents or create new ones tuned to your specific workflows across finance, legal, healthcare, retail, and beyond.",
                    image: "/river/assistants.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Integrations",
                    sub_title: "Seamlessly connect internal and external systems",
                    text: "Our LLM tools integrate effortlessly with your enterprise systems, ensuring smooth, secure, and intelligent automation across your entire workflow.",
                    image: "/river/integrations.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Teams",
                    sub_title: "Seamless Integration for Enhanced Collaboration",
                    text: "Empower your teams with AI that works where they do. Bionic integrates
                        seamlessly into your workflows, providing advanced capabilities without sacrificing security. 
                        Your data stays private, enabling trustworthy collaboration and innovation.",
                    image: "/river/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Observability and Auditability",
                    sub_title: "Stay in Control with Detailed Insights",
                    text: "Monitor usage, track interactions, and ensure compliance with robust observability and
                        auditability tools. Transparency and accountability are built right into Bionic.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                BionicFeatures {}

                Testamonial1 { }

                Benefits {
                    title: "Benefits",
                    subtitle: "AI Your Teams Will Actually Use — and Trust",
                    benefit1: "Accelerate Generative AI Adoption",
                    benefit1_desc: "Boost productivity with a solution that's simple to implement and use securely.",
                    benefit2: "Custom AI Assistants (Agentic RAG)",
                    benefit2_desc: "Utilize your data to create AI assistants that deliver smarter, tailored responses.",
                    benefit3: "Data Compliance and Auditability",
                    benefit3_desc: "Enjoy the advantages of generative AI with robust data governance and compliance.",
                }

                Faq {
                    questions: vec![
                        FaqText {
                            question: String::from("How does Bionic ensure data privacy?"),
                            answer: String::from("Bionic runs entirely within your environment, meaning your data never leaves your control. Unlike traditional AI models, there's no need to send information to external servers, eliminating the risk of leaks or unauthorized access."),
                        },
                        FaqText {
                            question: String::from("Is Bionic as powerful as Chat-GPT?"),
                            answer: String::from("Yes! Bionic delivers the same advanced AI capabilities as Chat-GPT, with the added advantage of running securely within your infrastructure. You get the full power of GPT without compromising privacy or control."),
                        },
                        FaqText {
                            question: String::from("Can Bionic be tailored to my specific needs?"),
                            answer: String::from("Absolutely. Bionic allows you to customize and fine-tune the AI using your own data, ensuring it provides accurate, context-aware insights and performs tasks specific to your business requirements."),
                        },
                        FaqText {
                            question: String::from("How do I monitor and manage usage?"),
                            answer: String::from("Bionic includes powerful observability and auditability features. You can track usage, monitor performance, and ensure compliance with detailed logs and insights into how the AI is being used."),
                        },
                        FaqText {
                            question: String::from("Is Bionic suitable for regulated industries?"),
                            answer: String::from("Yes. Bionic is designed with security and compliance in mind, making it ideal for industries with strict data protection requirements. It keeps sensitive information private while meeting regulatory standards."),
                        },
                    ]
                }

                Security {
                }
            }
            Footer {
            }
        }
    };

    crate::render(page)
}
