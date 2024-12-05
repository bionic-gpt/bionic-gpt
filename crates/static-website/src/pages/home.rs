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
use std::fs::File;
use std::io::Write;

pub async fn generate() {
    let html = crate::render(HomePage).await;

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "p-5 mt-24 flex flex-col items-center",
                VideoHero {
                    video: "https://www.youtube.com/embed/slRiOOM17tM?si=yBb5noZUF44ZIo70",
                    title: "The #1 Privacy Focused Enterprise Chat-GPT Solution.",
                    subtitle: "Enjoy the benefits of generative AI and keep data privacy and compliance in check.",
                    claim: "100's of installations globally."
                }
                Customers {
                    class: "mt-24"
                }

                ProblemSolution {
                    class: "mt-24",
                    image: "/landing-page/private-deployment.svg",
                    title: "How do you get the benefits of Chat-GPT and keep your data private?",
                    problem: "Chat-GPT offers incredible potential, but sending sensitive data to external servers, exposes you to risks like breaches and unauthorized access.
                        For businesses handling private or regulated data, this trade-off is simply unacceptable.",
                    solution: "Bionic-GPT offers a solution. We provide the power of Chat-GPT without the risks of data leakage by running securely within your environment.
                        Enjoy advanced AI capabilities and keep your data private, compliant, and fully under your control."
                }

                Benefits {
                    class: "mt-24",
                    title: "Benefits",
                    subtitle: "Rapidly Deploy Generative AI",
                    benefit1: "Easy to Deploy",
                    benefit1_desc: "Increase productivity across your whole organisation in a secure way.",
                    benefit2: "AI Assistants",
                    benefit2_desc: "Assistants allow you to use your data to get better answers.",
                    benefit3: "Data Compliance",
                    benefit3_desc: "All the benefits of Gen AI and security around data governance.",
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Data Governance",
                    sub_title: "Everything you Need",
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
                    sub_title: "AI Assistants Powered by Your Data",
                    text: "Transform your data into a competitive advantage by building AI assistants
                        tailored to your needs. With Bionic-GPT, 
                        you can train AI on your unique datasets, 
                        enabling it to provide accurate, context-aware insights and automate 
                        tasks specific to your business. All of this happens securely within your environment, ensuring your data remains private while unlocking the full potential of AI.",
                    image: "/landing-page/assistants.png",
                    flip: true
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Teams",
                    sub_title: "Bring AI to Your Teams, Securely ",
                    text: "Empower your teams with AI that works where they do. Bionic-GPT integrates
                        seamlessly into your workflows, providing advanced capabilities without 
                        sacrificing security. Your data stays private, enabling 
                        collaboration and innovation you can trust. ",
                    image: "/landing-page/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    class: "mt-24",
                    title: "Observability",
                    sub_title: "Observability and Auditability",
                    text: "Stay in control with detailed insights into your AI's activity.
                        Bionic-GPT offers robust observability and auditability tools, 
                        allowing you to monitor usage, track interactions, and 
                        ensure compliance with ease. Transparency and accountability, built right in.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                Features {
                    class: "mt-24",
                    title: "Bionic-GPT Features",
                    description: "A fully implemented solution for all your needs",
                    features: vec![
                        Feature {
                            title: String::from("No Code Rag"),
                            description: String::from("Allow users to create RAG pipelines in minutes"),
                        },
                        Feature {
                            title: String::from("Team-based permissions"),
                            description: String::from("Your teams know best which data is suitable for the AI."),
                        },
                        Feature {
                            title: String::from("Full Observability"),
                            description: String::from("Dashboards that give you insights into usage and compliance."),
                        },
                        Feature {
                            title: String::from("Cost Control"),
                            description: String::from("Set limits by users and teams and ensure fair usage."),
                        },
                        Feature {
                            title: String::from("Fully Encrypted"),
                            description: String::from("Encryption at rest, in transport and at run time."),
                        },
                        Feature {
                            title: String::from("Scalable"),
                            description: String::from("Run natively on Kubernetes for maximum scalability."),
                        },
                    ]
                }

                Testamonials {
                    text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic-GPT’s support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
                    job1: "Data Scientist",
                    person1: "Emmat",
                    text2: "Bionic-GPT’s observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
                    job2: "Compliance Officer",
                    person2: "Patrick",
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
    }
}
