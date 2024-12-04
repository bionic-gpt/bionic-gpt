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
                    title: "The #1 Enterprise Generative AI and Data Privacy Solution.",
                    subtitle: "We are a Chat-GPT replacement focused on data privacy and compliance.",
                    claim: "100's of installations globally."
                }
                Customers {}

                ProblemSolution {
                    image: "/landing-page/private-deployment.svg",
                    title: "How do you get the benefits of AI and keep your data private?",
                    subtitle: "A Chat-GPT Replacement Without The Data Leakage",
                }

                Benefits {
                    title: "Benefits",
                    subtitle: "More benefits",
                    benefit1: "Military grade encryption and data security best practices",
                    benefit1_desc: "A Chat-GPT Replacement Without The Data Leakage",
                    benefit2: "Military grade encryption and data security best practices",
                    benefit2_desc: "A Chat-GPT Replacement Without The Data Leakage",
                    benefit3: "Military grade encryption and data security best practices",
                    benefit3_desc: "A Chat-GPT Replacement Without The Data Leakage",
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/bionic-console.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "AI Assistants",
                    text: "Leverage your existing company knowledge to automate tasks like customer support,
        lead qualification, and RFP processing and much more.",
                    image: "/landing-page/assistants.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Data Governance",
                    sub_title: "A Chat-GPT Replacement Without The Data Leakage",
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
