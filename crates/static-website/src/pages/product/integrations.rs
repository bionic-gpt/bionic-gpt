use crate::components::footer::Footer;
use crate::components::hero::Hero;
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

                Hero {
                    title: "Here",
                    subtitle: "There"
                }

                ImageFeature {
                    title: "String".to_string(),
                    sub_title: "String".to_string(),
                    text: "String".to_string(),
                    title1: "String".to_string(),
                    text1: "String".to_string(),
                    title2: "String".to_string(),
                    text2: "String".to_string(),
                    title3: "String".to_string(),
                    text3: "String".to_string(),
                    image: "/product/integrations.svg"
                }
            }

            Footer {
                extra_class: "mt-24"
            }
        }
    };

    crate::render(page)
}
