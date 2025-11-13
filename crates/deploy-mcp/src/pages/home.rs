use dioxus::prelude::*;

use crate::components::customer_logos::Customers;
use crate::components::features::{Feature, Features};
use crate::components::hero::Hero;
use crate::components::testamonials::Testamonials;
use crate::components::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
};
use crate::layouts::layout::Layout;
use crate::routes;

pub fn home_page() -> String {
    let features = vec![
        Feature {
            title: "Curated server catalog".to_string(),
            description:
                "Launch popular MCP servers—or bring your own binary—with a single click.".to_string(),
            icon: "/features/systems.svg".to_string(),
        },
        Feature {
            title: "Managed credentials".to_string(),
            description:
                "Store API keys and OAuth tokens securely with scoped environment variables per server.".to_string(),
            icon: "/features/encryption.svg".to_string(),
        },
        Feature {
            title: "Instant connection URLs".to_string(),
            description:
                "Share ready-to-use MCP endpoints with teammates—no networking setup required.".to_string(),
            icon: "/features/graph.svg".to_string(),
        },
    ];

    let testimonials = rsx! {
        Testamonials {
            text1: "Deploy MCP let us spin up the blockchain server we needed for a hackathon in under ten minutes.",
            job1: "Developer Advocate", person1: "Sasha", img1: "https://placehold.co/96x96",
            text2: "Sharing Deploy's remote MCP URLs made onboarding our analysts trivial—no local setup required.",
            job2: "Data Engineer", person2: "Marcus", img2: "https://placehold.co/96x96",
            class: None,
        }
    };

    let body = rsx! {
        div {
            class: "mt-16 grid gap-y-36",
            section { class: "mx-auto lg:max-w-5xl p-6 text-center",
                Hero {
                    title: "Run managed MCP servers in minutes".to_string(),
                    subtitle: "Deploy MCP hosts Model Context Protocol servers for you, handles authentication, and gives you shareable connection URLs for any client.".to_string(),
                    cta_label: Some("Get started".to_string()),
                    cta_href: Some(crate::routes::SIGN_IN_UP.to_string()),
                }
            }

            Customers {
            }

            section { class: "mx-auto lg:max-w-5xl p-6",
                Features {
                    features: features.clone(),
                    title: "Why builders choose Deploy MCP".to_string(),
                    description: "Everything you need to run Model Context Protocol servers without provisioning infrastructure.".to_string(),
                    class: Some("".to_string()),
                }
            }
            section { class: "mx-auto lg:max-w-5xl p-6",
                div {
                    class: "grid gap-6 md:grid-cols-2 items-center",
                    img { class: "rounded-lg shadow-lg", src: "/docs/mcp-servers.png", alt: "Deploy MCP server catalog" }
                    div {
                        h2 { class: "text-3xl font-bold", "Launch your first server fast" }
                        p { class: "mt-4 text-lg", "Follow the four-step quickstart from our docs to go from zero to a running MCP endpoint." }
                        ol {
                            class: "mt-6 space-y-3 text-left list-decimal list-inside",
                            li {
                                strong { "Pick a server." }
                                " Browse the Deploy catalog and choose a curated MCP server to run."
                            }
                            li {
                                strong { "Configure credentials." }
                                " Add API keys or OAuth details right in the console—secrets stay scoped to that server."
                            }
                            li {
                                strong { "Provision instantly." }
                                " Hit \"Run MCP Server\" and Deploy starts the infrastructure for you."
                            }
                            li {
                                strong { "Copy the connection URL." }
                                " Drop the generated remote URL into your MCP-compatible client and start building."
                            }
                        }
                    }
                }
            }
            section { class: "mx-auto lg:max-w-5xl p-6", {testimonials} }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-connection-url.png".to_string(),
            cta: "Open the docs".to_string(),
            cta_url: routes::docs::Index {}.to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Deploy".to_string(),
            description: "Deploy MCP hosts Model Context Protocol servers with managed auth and shareable URLs.".to_string(),
            url: Some("https://deploy.run/".to_string()),
            section: crate::components::navigation::Section::Home,
            mobile_menu: None,
            image: Some("/docs/mcp-servers.png".to_string()),
            children: body,
        }
    };

    crate::render(page)
}
