use crate::marketing::{
    benefits::Benefits, features::BionicFeatures, footer::Footer, hero::Hero,
    testamonials::Testamonial2,
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn partners_page() -> String {
    let page = rsx! {
        Layout {
            title: "Partners",
            description: "Partners",
            section: Section::Partners,
            div {
                class: "px-4 md:px-0 w-full lg:max-w-5xl mt-16 md:mt-36 mx-auto grid gap-y-36",

                Hero {
                    title: "Become a Bionic Partner",
                    subtitle: "Unlock Revenue with Secure, Enterprise-Grade AI Solutions",
                    cta_label: "Book a Call",
                    cta_href: crate::routes::marketing::Contact {}.to_string()
                }

                Benefits {
                    title: "Partners",
                    subtitle: "Why Partner with Us?",
                    benefit1: "Revenue Growth",
                    benefit1_desc: "Earn from licensing new users, support, and upgrades,
                        while also providing AI consulting, training, and development services.",
                    benefit2: "In-Demand Solution",
                    benefit2_desc: "Our platform’s private, secure deployment model opens doors
                        to businesses prioritising data privacy and compliance.",
                    benefit3: "End-to-End Support",
                    benefit3_desc: "Get onboarding assistance and ongoing technical
                        support to ensure a seamless experience for you and your clients.",
                }

                BionicFeatures {}

                Testamonial2 {}

                section {
                    div {
                        class: "mt-10 flex flex-col items-center",
                        hr { class: "w-full mb-4" }
                        a {
                            href: "/contact",
                            class: "btn btn-secondary btn-outline",
                            "Book a Call"
                        }
                    }
                }
            }
        }
        Footer {
            links: footer_links()
        }
    };

    crate::render(page)
}
