use dioxus::prelude::*;

use crate::components::customer_logos::Customers;
use crate::components::features::{Feature, Features};
use crate::components::hero::Hero;
use crate::components::testamonials::Testamonials;
use crate::components::{extra_footer::ExtraFooter, footer::Footer};
use crate::layouts::layout::Layout;
use crate::routes;

pub fn home_page() -> String {
    let features = vec![
        Feature {
            title: "Model observability".to_string(),
            description: "Track latency, cost, and quality with out-of-the-box dashboards."
                .to_string(),
            icon: "https://placehold.co/48x48".to_string(),
        },
        Feature {
            title: "Governed autonomy".to_string(),
            description: "Deploy assistants with policy controls and scoped credentials."
                .to_string(),
            icon: "https://placehold.co/48x48".to_string(),
        },
        Feature {
            title: "Unified orchestration".to_string(),
            description: "Mix commercial and self-hosted models without touching infrastructure."
                .to_string(),
            icon: "https://placehold.co/48x48".to_string(),
        },
    ];

    let testimonials = rsx! {
        Testamonials {
            text1: "Deploy keeps our AI workflows compliant without slowing experimentation.",
            job1: "Head of Platform", person1: "Jamie", img1: "https://placehold.co/96x96",
            text2: "We shipped internal copilots twice as fast after moving to Deploy's automation tooling.",
            job2: "Director of Operations", person2: "Priya", img2: "https://placehold.co/96x96",
            class: None,
        }
    };

    let body = rsx! {
        main {
            class: "mt-16 grid gap-y-36",
            section { class: "mx-auto lg:max-w-5xl p-6 text-center",
                Hero {
                    title: "Safely and Securely deploy MCP servers".to_string(),
                    subtitle: "Production-ready MCP servers that extend AI capabilities through file access, database connections, API integrations, and other contextual services.".to_string(),
                }
            }

            Customers {
            }

            section { class: "mx-auto lg:max-w-5xl p-6",
                Features {
                    features: features.clone(),
                    title: "Why teams choose Deploy".to_string(),
                    description: "The Deploy platform ships with the controls enterprises expect.".to_string(),
                    class: Some("".to_string()),
                }
            }
            section { class: "mx-auto lg:max-w-5xl p-6",
                div {
                    class: "grid gap-6 md:grid-cols-2 items-center",
                    img { class: "rounded-lg shadow-lg", src: "https://placehold.co/540x360", alt: "Deploy playbooks" }
                    div {
                        h2 { class: "text-3xl font-bold", "Automate the boring work" }
                        p { class: "mt-4 text-lg", "Deploy orchestrates retrieval, function calling, and approvals so assistants can close the loop." }
                        ul {
                            class: "mt-6 space-y-2 text-left",
                            li { "• Trigger workflows from chat, APIs, or scheduled events." }
                            li { "• Route requests through approval queues when humans need to review." }
                            li { "• Log every decision so compliance teams stay informed." }
                        }
                    }
                }
            }
            section { class: "mx-auto lg:max-w-5xl p-6", {testimonials} }
            section { class: "mx-auto lg:max-w-5xl p-6 text-center",
                ExtraFooter {
                    title: "Ready to orchestrate production AI?".to_string(),
                    image: "https://placehold.co/600x360".to_string(),
                    cta: "Schedule a Deploy demo".to_string(),
                    cta_url: routes::marketing::Contact {}.to_string(),
                }
            }
        }
        Footer { }
    };

    let page = rsx! {
        Layout {
            title: "Deploy".to_string(),
            description: "Deploy helps platform teams launch AI assistants with governance and observability.".to_string(),
            section: crate::components::navigation::Section::Home,
            mobile_menu: None,
            image: Some("https://placehold.co/1200x630".to_string()),
            children: body,
        }
    };

    crate::render(page)
}
