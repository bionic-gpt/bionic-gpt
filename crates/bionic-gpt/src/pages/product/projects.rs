use crate::marketing::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    features::BionicFeatures,
    footer::Footer,
    layout::{PageContainer, PageRhythm, PageWidth},
    product_hero::ProductHero,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn page() -> String {
    let page = rsx! {
        Layout {
            title: "Projects for Enterprise AI",
            description: "Organize related AI chats, instructions, attachments, and history with Bionic Projects.",
            mobile_menu: None,
            section: Section::Home,

            PageContainer { width: PageWidth::Wide, rhythm: Some(PageRhythm::Product), class: Some("mt-16 pb-16 md:mt-24".to_string()),

                ProductHero {
                    eyebrow: "Projects".to_string(),
                    title: "Keep related chats, instructions, and attachments together".to_string(),
                    subtitle: "Give ongoing work a durable, organized home with Bionic Projects.".to_string(),
                    claim: None,
                    supporting: None,
                    primary_cta: "Get started".to_string(),
                    primary_href: crate::routes::marketing::GoBionic {}.to_string(),
                    secondary_cta: Some("Contact us".to_string()),
                    secondary_href: Some(crate::routes::marketing::Contact {}.to_string()),
                    image: "/product/chat.png".to_string(),
                    image_alt: "Bionic project chat".to_string(),
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
                margin_top: "mt-0",
                links: footer_links()
            }
        }
    };

    crate::render(page)
}
