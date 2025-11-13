use dioxus::prelude::*;

use crate::components::{
    customer_logos::Customers,
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
    security::Security,
    testamonials::Testamonials,
};
use crate::layouts::layout::Layout;

pub fn contact_page() -> String {
    let testimonials = rsx! {
        Testamonials {
            text1: "Deploy MCP helped us deliver a compliant on-premise assistant without rebuilding our entire platform tooling.",
            job1: "Head of Platform", person1: "Lena", img1: "https://placehold.co/96x96",
            text2: "The operator integrates with our RBAC and observability stack, so our security team signed off quickly.",
            job2: "Director of Security", person2: "Rahul", img2: "https://placehold.co/96x96",
            class: Some("".to_string()),
        }
    };

    let body = rsx! {
        div {
            class: "mt-20 mx-auto space-y-16 lg:max-w-5xl p-6",

            section { class: "text-center space-y-4",
                h1 { class: "text-4xl font-extrabold", "Talk with the Deploy team" }
                h2 { class: "text-2xl font-semibold", "Design a secure MCP rollout with us" }
                p { class: "text-lg", "Share your objectives and our engineers will map the fastest way to production—cloud or on premise." }
                p { class: "text-base", "Grab time on our calendar and we’ll tailor a session for your environment." }
                a {
                    class: "btn btn-primary",
                    href: "https://calendly.com/bionicgpt",
                    "Book a Calendly session"
                }
            }

            section { class: "grid gap-6 md:grid-cols-3",
                div { class: "rounded-lg border p-6 text-left shadow-sm",
                    h3 { class: "text-xl font-semibold", "Platform assessments" }
                    p { class: "mt-3", "Audit how Deploy MCP fits your clusters, identity, and compliance controls." }
                }
                div { class: "rounded-lg border p-6 text-left shadow-sm",
                    h3 { class: "text-xl font-semibold", "On-premise pilots" }
                    p { class: "mt-3", "Plan proof-of-concept timelines, image mirroring strategies, and success metrics." }
                }
                div { class: "rounded-lg border p-6 text-left shadow-sm",
                    h3 { class: "text-xl font-semibold", "Enterprise support" }
                    p { class: "mt-3", "Review SLAs, incident response, and long-term partnership programs." }
                }
            }

            Customers {}

            section { class: "", {testimonials} }

            section { class: "", Security { class: Some("".to_string()) } }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-connection-url.png".to_string(),
            cta: "Open deployment docs".to_string(),
            cta_url: "/docs/on-premise/".to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Contact Deploy".to_string(),
            description: "Connect with the Deploy team to design your AI automation rollout.".to_string(),
            url: Some("https://deploy.run/contact".to_string()),
            section: crate::components::navigation::Section::Contact,
            mobile_menu: None,
            image: None,
            children: body,
        }
    };

    crate::render(page)
}
