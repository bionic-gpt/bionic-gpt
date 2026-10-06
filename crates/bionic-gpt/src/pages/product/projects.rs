use crate::marketing::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    features::BionicFeatures,
    footer::Footer,
    image_feature::ImageFeature,
    layout::{PageContainer, PageGutters, PageRhythm, PageWidth},
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

            PageContainer { width: PageWidth::Content, gutters: Some(PageGutters::Roomy), rhythm: Some(PageRhythm::Legacy), class: Some("mt-24".to_string()),

                ImageFeature {
                    title: "Keep related chats, instructions, and attachments together in Projects".to_string(),
                    sub_title: "Give ongoing work a durable, organized home".to_string(),
                    image: "/product/chat.png"
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
