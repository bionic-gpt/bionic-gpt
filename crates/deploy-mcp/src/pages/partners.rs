use dioxus::prelude::*;

use crate::components::footer::Footer;
use crate::layouts::layout::Layout;

pub fn partners_page() -> String {
    let body = rsx! {
        main {
            class: "mt-20 mx-auto lg:max-w-5xl p-6",
            section {
                class: "text-center",
                h1 { class: "text-4xl font-bold", "Deploy Partner Network" }
                p {
                    class: "mt-4 text-lg",
                    "Deploy works with solution partners to help customers modernize operations with AI automation."
                }
            }
            section {
                class: "mt-12",
                ul {
                    class: "grid gap-4 md:grid-cols-3",
                    li {
                        class: "rounded-lg border p-4 text-left",
                        strong { "Northwind Robotics" }
                        p { class: "mt-2 text-sm", "Deploy powers their manufacturing copilots." }
                    }
                    li {
                        class: "rounded-lg border p-4 text-left",
                        strong { "Atlas Airlines" }
                        p { class: "mt-2 text-sm", "Deploy handles their operations knowledge base." }
                    }
                    li {
                        class: "rounded-lg border p-4 text-left",
                        strong { "Brightside Health" }
                        p { class: "mt-2 text-sm", "Deploy automates intake triage and reporting." }
                    }
                }
            }
            section {
                class: "mt-12 text-center",
                a {
                    class: "btn btn-primary",
                    href: crate::routes::marketing::Contact {}.to_string(),
                    "Become a partner"
                }
            }
        }
        Footer { }
    };

    let page = rsx! {
        Layout {
            title: "Deploy Partner Network".to_string(),
            description: "Work with Deploy to deliver trusted AI automation to your clients.".to_string(),
            section: crate::components::navigation::Section::Partners,
            mobile_menu: None,
            image: None,
            children: body,
        }
    };

    crate::render(page)
}
