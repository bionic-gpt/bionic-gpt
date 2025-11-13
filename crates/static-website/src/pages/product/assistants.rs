use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::features::BionicFeatures;
use crate::components::footer::Footer;
use crate::components::image_feature::ImageFeature;
use crate::components::navigation::Section;
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
                class: "lg:max-w-5xl p-5 mt-24 mx-auto grid gap-y-48",

                ImageFeature {
                    title: "Assistants can connect to your business knowledge and improve AI intelligence".to_string(),
                    sub_title: "Open up your private data to secure AI".to_string(),
                    image: "/product/assistants.png"
                }

                BionicFeatures {}
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
