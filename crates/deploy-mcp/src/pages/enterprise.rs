use dioxus::prelude::*;

use crate::components::customer_logos::Customers;
use crate::components::features::{Feature, Features};
use crate::components::hero::Hero;
use crate::components::security::Security;
use crate::components::{
    extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
    footer::Footer,
};
use crate::layouts::layout::Layout;

pub fn enterprise_page() -> String {
    let features = vec![
        Feature {
            title: "Run inside your own perimeter".to_string(),
            description: "Install Deploy MCP on Kubernetes clusters you already trust and keep prompts, credentials, and logs within your security boundary.".to_string(),
            icon: "/features/kubernetes.svg".to_string(),
        },
        Feature {
            title: "Hardened supply chain".to_string(),
            description: "Signed container images and reproducible builds give your security teams the provenance they expect for regulated environments.".to_string(),
            icon: "/features/encryption.svg".to_string(),
        },
        Feature {
            title: "Enterprise identity everywhere".to_string(),
            description: "Connect Deploy MCP to your existing SSO, SCIM, and RBAC policies so only the right teams can launch or manage servers.".to_string(),
            icon: "/features/team.svg".to_string(),
        },
    ];

    let doc_sections = [
        (
            "Operator install",
            "Deploy the controller with Helm or Kustomize, complete with CRDs and permissions.",
            "/docs/on-premise/install-operator/",
        ),
        (
            "AWS reference",
            "Follow the EKS-focused guide to wire in load balancing, storage classes, and logging.",
            "/docs/on-premise/aws/",
        ),
        (
            "Google Cloud reference",
            "Use the GKE/Kubernetes Engine walkthrough to mirror networking and IAM best practices.",
            "/docs/on-premise/gcloud/",
        ),
        (
            "Identity & SSO",
            "Connect SAML or OIDC providers and sync teams automatically via SCIM.",
            "/docs/on-premise/sso/",
        ),
        (
            "Role-based access",
            "Map Deploy MCP roles to Kubernetes namespaces and enforce least privilege.",
            "/docs/on-premise/rbac/",
        ),
        (
            "Backups & disaster recovery",
            "Schedule backups for Postgres and object storage with tested restore procedures.",
            "/docs/on-premise/backups/",
        ),
        (
            "Upgrade playbook",
            "Plan rolling updates with pinned images and preflight checks across environments.",
            "/docs/on-premise/upgrades/",
        ),
        (
            "Licensing options",
            "Choose the deployment model and support agreement that matches your governance needs.",
            "/docs/on-premise/licencing/",
        ),
    ];

    let body = rsx! {
        div {
            class: "mt-16 grid gap-y-36",
            section { class: "mx-auto lg:max-w-5xl p-6 text-center",
                Hero {
                    title: "Deploy MCP on your infrastructure".to_string(),
                    subtitle: "Install our operator in your Kubernetes clusters to keep sensitive workloads on premise while giving teams the same managed MCP experience.".to_string(),
                }
            }

            Customers {}

            section { class: "mx-auto lg:max-w-5xl p-6",
                Features {
                    features: features.clone(),
                    title: "Built for security-conscious enterprises".to_string(),
                    description: "Everything in Deploy MCP can run in your environment with the controls your platform team already owns.".to_string(),
                    class: Some("".to_string()),
                }
            }

            section { class: "mx-auto lg:max-w-5xl p-6",
                div { class: "grid gap-8 md:grid-cols-2 md:items-center",
                    div {
                        h2 { class: "text-3xl font-bold", "Kubernetes-native architecture" }
                        p { class: "mt-4 text-lg", "Our operator keeps MCP servers declarative and auditable. Use namespaces, network policies, and your existing GitOps tooling without special cases." }
                        ul {
                            class: "mt-6 space-y-3 text-left list-disc list-inside text-base",
                            li { strong { "Registry credentials" } " — Configure private pull secrets once and reconcile them automatically." }
                            li { strong { "Environment isolation" } " — Segment assistants per team with namespace boundaries and resource quotas." }
                            li { strong { "Observability hooks" } " — Ship logs and metrics into the same stack you use for every other workload." }
                        }
                        a {
                            class: "btn btn-link mt-6 px-0",
                            href: "/docs/on-premise/",
                            "Read the on-premise installation guide"
                        }
                    }
                    img {
                        class: "rounded-lg shadow-lg",
                        src: "/docs/mcp-servers.png",
                        alt: "Deploy MCP architecture"
                    }
                }
            }

            section { class: "mx-auto lg:max-w-5xl p-6",
                h2 { class: "text-3xl font-bold text-center", "Enterprise deployment playbooks" }
                p { class: "mt-4 text-lg text-center", "Dive into the step-by-step guides our team maintains for regulated deployments across infrastructure, identity, and operations." }
                div {
                    class: "mt-10 grid gap-6 md:grid-cols-2",
                    for (title, blurb, href) in doc_sections.iter() {
                        a {
                            class: "block rounded-lg border border-base-300 p-6 shadow-sm transition hover:border-primary hover:shadow-md",
                            href: "{href}",
                            h3 { class: "text-xl font-semibold", "{title}" }
                            p { class: "mt-3 text-base", "{blurb}" }
                            span { class: "mt-4 inline-flex items-center text-primary", "Read the guide →" }
                        }
                    }
                }
            }

            section { class: "mx-auto lg:max-w-5xl p-6",
                Security { class: Some("".to_string()) }
            }
        }
        ExtraFooter {
            title: EXTRA_FOOTER_TITLE.to_string(),
            image: "/docs/mcp-connection-url.png".to_string(),
            cta: "Open installation docs".to_string(),
            cta_url: "/docs/on-premise/".to_string(),
        }
        Footer { margin_top: "mt-0" }
    };

    let page = rsx! {
        Layout {
            title: "Deploy MCP for Enterprise".to_string(),
            description: "Run Deploy MCP on premise or in your private cloud with hardened supply chains and enterprise controls.".to_string(),
            url: Some("https://deploy.run/enterprise".to_string()),
            section: crate::components::navigation::Section::Enterprise,
            mobile_menu: None,
            image: Some("/docs/mcp-servers.png".to_string()),
            children: body,
        }
    };

    crate::render(page)
}
