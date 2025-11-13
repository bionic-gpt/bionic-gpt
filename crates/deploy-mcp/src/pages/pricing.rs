use dioxus::prelude::*;

use crate::components::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
};
use crate::layouts::layout::Layout;

pub fn pricing_page() -> String {
    let body = rsx! {
        div {
            class: "mt-20 mx-auto lg:max-w-5xl p-6",
            section {
                class: "text-center",
                h1 { class: "text-4xl font-bold", "Deploy Pricing" }
                p {
                    class: "mt-4 text-lg",
                    "Usage-based pricing so you only pay for the API calls you make."
                }
            }
            section {
                class: "mt-12 grid gap-6 md:grid-cols-3",
                PricingCard {
                    title: "Free Trial",
                    price: "Free",
                    description: "Start building with 100 API calls every month to explore Deploy.",
                    features: vec![
                        "No credit card required",
                        "Full access to MCP server catalog",
                        "Community support",
                    ],
                    cta_href: crate::routes::SIGN_IN_UP.to_string(),
                }
                PricingCard {
                    title: "Pay as you go",
                    price: "$0.05 per call",
                    description: "Scale usage-based billing as you automate more workflows.",
                    features: vec![
                        "Unlimited production assistants",
                        "Real-time usage analytics",
                        "Email support",
                    ],
                    cta_href: crate::routes::SIGN_IN_UP.to_string(),
                }
                PricingCard {
                    title: "Enterprise",
                    price: "Custom",
                    description: "Deploy at scale with dedicated support and advanced security controls.",
                    features: vec![
                        "Single Sign-On (SSO) & SCIM provisioning",
                        "Dedicated success manager",
                        "Custom contracts and SLAs",
                    ],
                    cta_href: crate::routes::marketing::Contact {}.to_string(),
                }
            }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-servers.png".to_string(),
            cta: "Get Started".to_string(),
            cta_url: crate::routes::marketing::Index {}.to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Deploy Pricing".to_string(),
            description: "Choose a Deploy plan that fits your automation roadmap.".to_string(),
            url: Some("https://deploy.run/pricing".to_string()),
            section: crate::components::navigation::Section::Pricing,
            mobile_menu: None,
            image: None,
            children: body,
        }
    };

    crate::render(page)
}

#[component]
fn PricingCard(
    title: &'static str,
    price: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
    cta_href: String,
) -> Element {
    rsx! {
        div {
            class: "rounded-lg border p-6 shadow-sm flex flex-col",
            h2 { class: "text-2xl font-semibold", "{title}" }
            p { class: "mt-4 text-3xl font-bold", "{price}" }
            p { class: "mt-2 text-sm text-base-content/70", "{description}" }
            ul {
                class: "mt-4 flex-1 space-y-2 text-left",
                for feature in features {
                    li { "• {feature}" }
                }
            }
            a {
                class: "btn btn-primary mt-6",
                href: cta_href,
                "Get started"
            }
        }
    }
}
