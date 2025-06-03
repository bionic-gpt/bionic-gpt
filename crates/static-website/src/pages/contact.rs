use crate::components::extra_footer::ExtraFooter;
use crate::components::footer::Footer;
use crate::components::navigation::Section;
use crate::components::security::Security;
use crate::components::team::Team;
use crate::components::testamonials::Testamonials;
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

                Testamonials {
                    text1: "Having the flexibility to use the best model for the job has been a game-changer. Bionic-GPT’s support for multiple models ensures we can tailor solutions to specific challenges, delivering optimal results every time.",
                    job1: "Data Scientist",
                    person1: "Emma",
                    img1: "https://dummyimage.com/106x106",
                    text2: "Bionic-GPT’s observability feature, which logs all messages into and out of the models, has been critical for ensuring compliance in our organization. It gives us peace of mind and robust accountability.",
                    job2: "Compliance Officer",
                    person2: "Patrick",
                    img2: "https://dummyimage.com/106x106",
                }

                Security {
                    class: "mt-24"
                }
            }

            ExtraFooter {
                title: "The secure open source Chat-GPT replacement
                that runs in a trusted execution environment for
                maximum data security and compliance",
                image: "/landing-page/bionic-console.png",
                cta: "Find out more",
                cta_url: crate::routes::marketing::Index {}.to_string()
            }
            Footer {
                extra_class: ""
            }
        }
    };

    crate::render(page)
}
