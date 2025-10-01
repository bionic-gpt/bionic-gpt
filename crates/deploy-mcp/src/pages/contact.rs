use dioxus::prelude::*;

use crate::components::{extra_footer::ExtraFooter, footer::Footer};
use crate::layouts::layout::Layout;

pub fn contact_page() -> String {
    let body = rsx! {
        div {
            class: "mt-20 mx-auto lg:max-w-3xl p-6",
            section {
                h1 { class: "text-4xl font-bold", "Talk with Deploy" }
                p {
                    class: "mt-4 text-lg",
                    "Tell us about your use case and we'll schedule a walkthrough with the Deploy team."
                }
                form {
                    class: "mt-8 space-y-4",
                    fieldset {
                        class: "flex flex-col",
                        label { class: "mb-1 font-semibold", "Work email" }
                        input { class: "input input-bordered", "type": "email", required: "true", placeholder: "you@company.com" }
                    }
                    fieldset {
                        class: "flex flex-col",
                        label { class: "mb-1 font-semibold", "Company" }
                        input { class: "input input-bordered", "type": "text", required: "true", placeholder: "Company" }
                    }
                    fieldset {
                        class: "flex flex-col",
                        label { class: "mb-1 font-semibold", "How can we help?" }
                        textarea { class: "textarea textarea-bordered", rows: "4", placeholder: "Share what you want to build" }
                    }
                    button { class: "btn btn-primary", "Request a call" }
                }
            }
        }
        ExtraFooter {
            title: "See Deploy in action in under five minutes".to_string(),
            image: "/docs/mcp-servers.png".to_string(),
            cta: "Get Started".to_string(),
            cta_url: crate::routes::marketing::Index {}.to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Contact Deploy".to_string(),
            description: "Connect with the Deploy team to design your AI automation rollout.".to_string(),
            section: crate::components::navigation::Section::Contact,
            mobile_menu: None,
            image: None,
            children: body,
        }
    };

    crate::render(page)
}
