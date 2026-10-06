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
            title: "Enterprise Generative AI",
            description: "The Industry Standard For Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Home,

            PageContainer { width: PageWidth::Wide, rhythm: Some(PageRhythm::Product), class: Some("mt-16 pb-16 md:mt-24".to_string()),

                ProductHero {
                    eyebrow: "Developers".to_string(),
                    title: "Distribute AI power to your data scientists and developers".to_string(),
                    subtitle: "Manage resource usage and access controls".to_string(),
                    claim: None,
                    supporting: None,
                    primary_cta: "Get started".to_string(),
                    primary_href: crate::routes::SIGN_IN_UP.to_string(),
                    secondary_cta: Some("Contact us".to_string()),
                    secondary_href: Some(crate::routes::marketing::Contact {}.to_string()),
                    image: "/product/developers.png".to_string(),
                    image_alt: "Bionic developer controls".to_string(),
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
