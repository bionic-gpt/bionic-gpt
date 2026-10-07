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
                    eyebrow: "Datasets".to_string(),
                    title: "Connect private knowledge to sovereign AI".to_string(),
                    subtitle: "Organise documents, policies, runbooks, and knowledge sources for grounded AI workflows".to_string(),
                    claim: None,
                    supporting: None,
                    primary_cta: "Explore datasets".to_string(),
                    primary_href: "/docs/guides/datasets/".to_string(),
                    secondary_cta: Some("Get started".to_string()),
                    secondary_href: Some(crate::routes::marketing::GoBionic {}.to_string()),
                    image: "/product/datasets.png".to_string(),
                    image_alt: "Bionic datasets workspace".to_string(),
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
