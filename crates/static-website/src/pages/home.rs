use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::faq_accordian::{Faq, FaqText};
use crate::components::features::{Feature, Features};
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::components::problem_solution::ProblemSolution;
use crate::components::security::Security;
use crate::components::small_image_feature::SmallImageFeature;
use crate::components::testamonials::Testamonials;
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
                class: "lg:max-w-5xl p-5 mt-8 md:mt-24 mx-auto",
                VideoHero {
                    video: "https://www.youtube.com/embed/slRiOOM17tM?si=yBb5noZUF44ZIo70",
                    title: "The all-in-one Private Generative AI platform — built for enterprise security, and control.",
                    subtitle: "Unlock the full potential of generative AI while keeping your data secure and compliant.",
                    claim: "and join hundreds of global installations!"
                }
                Customers {
                    class: "mt-24"
                }

                ProblemSolution {
                    class: "mt-24",
                    image: "/landing-page/private-deployment.svg",
                    title: "Leverage Generative AI Across Your Enterprise",
                    problem: "While Chat-GPT offers incredible potential, sending sensitive data to external servers
                        poses significant risks like breaches and unauthorized access. For businesses handling private or regulated data, 
                        this trade-off is unacceptable.",
                    solution: "Bionic-GPT provides the power of Chat-GPT without the risks of data leakage by running securely within your environment.
                        Enjoy advanced AI capabilities while keeping your data private, compliant, and fully under your control."
                }

                Benefits {
                    class: "mt-24",
                    title: "Benefits",
                    subtitle: "Rapidly Deploy Generative AI Across Your Organization",
                    benefit1: "Accelerate Generative AI Adoption",
                    benefit1_desc: "Boost productivity with a solution that's simple to implement and use securely.",
                    benefit2: "Custom AI Assistants (RAG)",
                    benefit2_desc: "Utilize your data to create AI assistants that deliver smarter, tailored responses.",
                    benefit3: "Data Compliance and Auditability",
                    benefit3_desc: "Enjoy the advantages of generative AI with robust data governance and compliance.",
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Data Governance",
                    sub_title: "Empower Your Teams with Secure AI",
                    text: "No learning curve, no confusion—just instant productivity.
                        Bionic-GPT features an intuitive user interface, 
                        so your teamscan get started immediately.
                        Familiarity means faster adoption and a seamless experience for everyone.",
                    image: "/landing-page/bionic-console.png",
                    flip: false
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Retrieval Augmented Generation",
                    sub_title: "Transform Your Data into a Competitive Advantage",
                    text: "Build AI assistants tailored to your needs by training on your unique datasets.
                        Bionic-GPT provides accurate, context-aware insights and automates tasks specific to your 
                        business—all securely within your environment.",
                    image: "/landing-page/assistants.png",
                    flip: true
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Teams",
                    sub_title: "Seamless Integration for Enhanced Collaboration",
                    text: "Empower your teams with AI that works where they do. Bionic-GPT integrates
                        seamlessly into your workflows, providing advanced capabilities without sacrificing security. 
                        Your data stays private, enabling trustworthy collaboration and innovation.",
                    image: "/landing-page/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Observability and Auditability",
                    sub_title: "Stay in Control with Detailed Insights",
                    text: "Monitor usage, track interactions, and ensure compliance with robust observability and
                        auditability tools. Transparency and accountability are built right into Bionic-GPT.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                Features {
                    class: "mt-24",
                    title: "Bionic-GPT Features",
                    description: "A comprehensive solution for all your AI needs.",
                    features: vec![
                        Feature {
                            title: String::from("No-Code RAG (Retrieval-Augmented Generation)"),
                            description: String::from("Create RAG pipelines in minutes without any coding."),
                        },
                        Feature {
                            title: String::from("Team-Based Permissions"),
                            description: String::from("Control data access and ensure security by allowing teams to manage permissions."),
                        },
                        Feature {
                            title: String::from("Full Observability"),
                            description: String::from("Gain insights into usage and compliance with detailed dashboards and logs."),
                        },
                        Feature {
                            title: String::from("Cost Control"),
                            description: String::from("Set usage limits by user and team to manage costs effectively."),
                        },
                        Feature {
                            title: String::from("Advanced Encryption"),
                            description: String::from("Ensure data security with encryption at rest, in transit, and during runtime."),
                        },
                        Feature {
                            title: String::from("Scalable Architecture"),
                            description: String::from("Built on Kubernetes for maximum scalability and reliability."),
                        },
                    ]
                }

                Testamonials {
                    class: "mt-24",
                    text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic-GPT’s support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
                    job1: "Data Scientist",
                    person1: "Emmat",
                    img1: "https://dummyimage.com/106x106",
                    text2: "Bionic-GPT’s observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
                    job2: "Compliance Officer",
                    person2: "Patrick",
                    img2: "https://dummyimage.com/106x106"
                }

                Faq {
                    class: "mt-24",
                    questions: vec![
                        FaqText {
                            question: String::from("How does Bionic-GPT ensure data privacy?"),
                            answer: String::from("Bionic-GPT runs entirely within your environment, meaning your data never leaves your control. Unlike traditional AI models, there’s no need to send information to external servers, eliminating the risk of leaks or unauthorized access."),
                        },
                        FaqText {
                            question: String::from("Is Bionic-GPT as powerful as Chat-GPT?"),
                            answer: String::from("Yes! Bionic-GPT delivers the same advanced AI capabilities as Chat-GPT, with the added advantage of running securely within your infrastructure. You get the full power of GPT without compromising privacy or control."),
                        },
                        FaqText {
                            question: String::from("Can Bionic-GPT be tailored to my specific needs?"),
                            answer: String::from("Absolutely. Bionic-GPT allows you to customize and fine-tune the AI using your own data, ensuring it provides accurate, context-aware insights and performs tasks specific to your business requirements."),
                        },
                        FaqText {
                            question: String::from("How do I monitor and manage usage?"),
                            answer: String::from("Bionic-GPT includes powerful observability and auditability features. You can track usage, monitor performance, and ensure compliance with detailed logs and insights into how the AI is being used."),
                        },
                        FaqText {
                            question: String::from("Is Bionic-GPT suitable for regulated industries?"),
                            answer: String::from("Yes. Bionic-GPT is designed with security and compliance in mind, making it ideal for industries with strict data protection requirements. It keeps sensitive information private while meeting regulatory standards."),
                        },
                    ]
                }

                Security {
                    class: "mt-24"
                }
            }
            Footer {}
        }
    };

    crate::render(page)
}
