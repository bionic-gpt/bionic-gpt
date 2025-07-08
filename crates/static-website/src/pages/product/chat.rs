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
                    title: "Chat privately with AI all data under your control.".to_string(),
                    sub_title: "Meet data governance controls and requirements".to_string(),
                    image: "/product/chat.png"
                }
            }

            Footer {
            }
        }
    };

    crate::render(page)
}
