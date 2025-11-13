use crate::components::customer_logos::Customers;
use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::features::BionicFeatures;
use crate::components::footer::Footer;
use crate::components::image_hero::ImageHero;
use crate::components::navigation::Section;
use crate::components::small_image_feature::SmallImageFeature;
use crate::components::testamonials::Testamonial2;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            div {
                class: "lg:max-w-5xl p-5 mt-24 mx-auto grid gap-y-24",

                ImageHero {
                    title: "The fastest way to build an AI assistant on your technical content.".to_string(),
                    image: "/solutions/chat-bot.png",
                    subtitle: "Add AI to your docs, product, and support flows to answer technical questions—in days, not months. ".to_string()
                }

                Customers {
                }

                SmallImageFeature {
                    title: "Agentic AI",
                    sub_title: "It's not easy to build AI assistants on your data",
                    text: "We manage all the complexity of Agentic RAG pipelines and you build no code assistants.",
                    image: "/river/assistants.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Integrations",
                    sub_title: "Connect assistants to anything",
                    text: "We are not just an Agentic RAG pipeline—we also support integrating with your support systems.",
                    image: "/river/integrations.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Teams",
                    sub_title: "Collaborate with Ease",
                    text: "Our Teams feature makes it simple for all your support teams to together in one place.",
                    image: "/river/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Observability",
                    sub_title: "AI Support at Scale",
                    text: "We provide monitoring tools to help identify performance bottlenecks and usage issues.",
                    image: "/landing-page/dashboard.png",
                    flip: true
                }

                BionicFeatures {}

                Testamonial2 { }
            }

            ExtraFooter {
                title: EXTRA_FOOTER_TITLE.to_string(),
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
            Footer {
                margin_top: "mt-0"
            }
        }
    };

    crate::render(page)
}
