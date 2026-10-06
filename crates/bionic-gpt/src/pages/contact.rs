use crate::marketing::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
    layout::{PageContainer, PageGutters, PageWidth},
    security::Security,
    team::Team,
    testamonials::Testamonial1,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn contact_page() -> String {
    let page = rsx! {
        Layout {
            title: "Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Contact,
            description: "The Industry Standard For Enterprise Generative AI",
            PageContainer { width: PageWidth::Content, gutters: Some(PageGutters::Roomy), class: Some("mt-8 md:mt-24".to_string()),
                section {
                    class: "p-5 text-center mb-12",
                    h1 {
                        class: "text-4xl font-extrabold mt-4",
                        "Our Team is Waiting to Hear From You"
                    }
                    h2 {
                        class: "text-2xl font-bold mt-4",
                        "Contact the Experts in Gen AI Deployments"
                    }
                    p {
                        class: "font-bold mt-4",
                        "Email founders (at) bionic-gpt.com"
                    }
                }

                Team {

                }

                Testamonial1 {}

                Security {
                    class: "mt-24"
                }
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
