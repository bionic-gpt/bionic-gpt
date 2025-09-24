use dioxus::prelude::*;

use crate::components::footer::Footer;
use crate::layouts::layout::Layout;

pub fn pricing_page() -> String {
    let body = rsx! {
        main {
            class: "mt-20 mx-auto lg:max-w-5xl p-6",
            section {
                class: "text-center",
                h1 { class: "text-4xl font-bold", "Deploy Pricing" }
                p {
                    class: "mt-4 text-lg",
                    "Simple plans that scale as your team ships more assistants."
                }
            }
            section {
                class: "mt-12 grid gap-6 md:grid-cols-3",
                PricingCard {
                    title: "Builder",
                    price: "Free",
                    description: "Launch pilots with up to 5 team members.",
                    features: vec![
                        "Unlimited development projects",
                        "Access to Deploy sandbox connectors",
                        "Community support",
                    ],
                }
                PricingCard {
                    title: "Growth",
                    price: "$299/mo",
                    description: "Production governance and automation.",
                    features: vec![
                        "Role-based access controls",
                        "Managed retrieval pipelines",
                        "Usage analytics and guardrails",
                    ],
                }
                PricingCard {
                    title: "Enterprise",
                    price: "Talk to us",
                    description: "Tailored deployments with dedicated support.",
                    features: vec![
                        "Private cloud or on-premise options",
                        "Custom SLAs",
                        "Enterprise integrations",
                    ],
                }
            }
        }
        Footer { }
    };

    let page = rsx! {
        Layout {
            title: "Deploy Pricing".to_string(),
            description: "Choose a Deploy plan that fits your automation roadmap.".to_string(),
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
) -> Element {
    rsx! {
        div {
            class: "rounded-lg border p-6 shadow-sm flex flex-col",
            h2 { class: "text-2xl font-semibold", "{title}" }
            p { class: "mt-4 text-3xl font-bold", "{price}" }
            p { class: "mt-2 text-sm text-gray-500", "{description}" }
            ul {
                class: "mt-4 flex-1 space-y-2 text-left",
                for feature in features {
                    li { "• {feature}" }
                }
            }
            a {
                class: "btn btn-primary mt-6",
                href: crate::routes::marketing::Contact {}.to_string(),
                "Contact sales"
            }
        }
    }
}
