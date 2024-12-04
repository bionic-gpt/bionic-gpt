use crate::components::benefits::Benefits;
use crate::components::customer_logos::Customers;
use crate::components::faq_accordian::Faq;
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
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "p-5 mt-24 flex flex-col items-center",
                VideoHero {
                    video: "https://www.youtube.com/embed/slRiOOM17tM?si=yBb5noZUF44ZIo70",
                    title: "The #1 Private Enterprise Chat-GPT Solution.",
                    subtitle: "Enjoy the benfits of generative AI and keep data privacy and compliance in check.",
                    claim: "100's of installations globally."
                }
                Customers {}

                ProblemSolution {
                    image: "/landing-page/private-deployment.svg",
                    title: "How do you get the benefits of Chat-GPT and keep your data private?",
                    problem: "Chat-GPT offers incredible potential, but sending sensitive data to external servers, exposes you to risks like breaches and unauthorized access.
                        For businesses handling private or regulated data, this trade-off is simply unacceptable.",
                    solution: "Bionic-GPT offers a solution. We provide the power of Chat-GPT without the risks of data leakage by running securely within your environment.
                        Enjoy advanced AI capabilities and keep your data private, compliant, and fully under your control."
                }

                Benefits {
                    title: "Benefits",
                    subtitle: "A full Chat-GPT replacement",
                    benefit1: "Have it Your Way",
                    benefit1_desc: "Install on your premise, in your private cloud or even our cloud.",
                    benefit2: "Everything You Need",
                    benefit2_desc: "A full solution including the full chat experience and AI assistants.",
                    benefit3: "Secure",
                    benefit3_desc: "No other soliution offers the same level of security.",
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Familiar User Interface",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/bionic-console.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Retrieval Augmented Generation",
                    sub_title: "AI Assistants",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/assistants.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Teams",
                    sub_title: "Sharing is Caring",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Observability",
                    sub_title: "Powerful Observability Features",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                Features {
                    title: "Bionic-GPT Features",
                    description: "A fully implemented solution for all your needs",
                    features
                }

                Testamonials {
                    text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic-GPT’s support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
                    job1: "Data Scientist",
                    person1: "Emma Trident",
                    text2: "Bionic-GPT’s observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
                    job2: "Compliance Officer",
                    person2: "Patrick O'leary",
                }

                Faq {}

                Security {}
            }
            Footer {}
        }
    }
}
