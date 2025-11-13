use crate::components::customer_logos::Customers;
use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::features::BionicFeatures;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
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

                Hero {
                    title: "Responsible and safe AI in education".to_string(),
                    subtitle: "At Bionic we believe AI has the power to fundamentally transform education for the better. We develop Bionic with powerful safeguards to ensure it remains a beneficial tool for students, educators, and administrators.".to_string()
                }

                Customers {
                }

                SmallImageFeature {
                    title: "Agentic AI",
                    sub_title: "Designed for true learning",
                    text: "We build tools that promote critical thinking and deep understanding over easy shortcuts.",
                    image: "/river/education_assistants.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Integrations",
                    sub_title: "Champion educational equity",
                    text: "We partner with institutions to make AI-powered education accessible to every student.",
                    image: "/river/integrations.png",
                    flip: true
                }

                SmallImageFeature {
                    title: "Teams",
                    sub_title: "Collaborate with Ease",
                    text: "Our Teams feature makes it simple for educators and staff to collaborate, share insights, and support students together in one place.",
                    image: "/river/teams.png",
                    flip: false
                }

                SmallImageFeature {
                    title: "Observability",
                    sub_title: "Supporting Student Wellbeing",
                    text: "We provide thoughtful monitoring tools to help identify students who may need additional support or care.",
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
