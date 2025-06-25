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
                    title: "Bring together your systems and knowledge and let AI automate business processes".to_string(),
                    sub_title: "Empower your teams and people with tools that boost their productivity".to_string(),
                    image: "/product/automations.png"
                }
            }

            Footer {
                extra_class: "mt-24"
            }
        }
    };

    crate::render(page)
}
