use crate::components::extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE};
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::components::security::Security;
use crate::components::team::Team;
use crate::components::testamonials::Testamonial1;
use crate::layouts::layout::Layout;
use dioxus::prelude::*;

pub fn contact_page() -> String {
    let page = rsx! {
        Layout {
            title: "Enterprise Generative AI",
            mobile_menu: None,
            section: Section::Contact,
            description: "The Industry Standard For Enterprise Generative AI",
            div {
                class: "lg:max-w-5xl p-5 mt-8 md:mt-24 mx-auto",
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
                    p {
                        class: "mt-4 mb-4",
                        "Or Schedule a Meeting with Calendly"
                    }
                    a {
                        class: "btn btn-primary",
                        href: "https://calendly.com/bionicgpt",
                        "Book a Call via Calendly"
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
                margin_top: "mt-0"
            }
        }
    };

    crate::render(page)
}
