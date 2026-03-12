use dioxus::prelude::*;
use ssg_whiz::FooterLinks;

#[component]
pub fn Footer(margin_top: Option<String>, links: FooterLinks) -> Element {
    let extra_class = if let Some(extra_class) = margin_top {
        extra_class
    } else {
        "mt-24".to_string()
    };

    if links.variant.as_deref() == Some("decision-luxe") {
        return rsx! {
            footer {
                class: "dl-footer {extra_class}",
                div {
                    class: "dl-footer-grid",
                    div {
                        h4 { "Decision Advantage" }
                        p {
                            class: "dl-lead",
                            style: "font-size:0.92rem;max-width:30ch;",
                            "Decision Advantage - Agentic decision support for command judgment."
                        }
                        div { class: "status", span { class: "pulse-dot" } "System Operational" }
                    }
                    nav {
                        h4 { "Platform" }
                        a { href: "/#hero", "Overview" }
                        a { href: "/#protocol", "Architecture" }
                    }
                    nav {
                        h4 { "Security" }
                        a { href: "/#manifesto", "Controls" }
                        a { href: "/#manifesto", "Assurance" }
                    }
                    nav {
                        h4 { "Contact" }
                        a { href: links.contact.clone(), "Schedule Demo" }
                        a { href: links.contact.clone(), "Inquiries" }
                    }
                }
            }
        };
    }

    rsx! {
        footer {
            class: "{extra_class} bg-neutral text-neutral-content p-10",
            div {
                class: "mx-auto lg:max-w-5xl flex flex-col md:flex-row justify-between",
                nav {
                    h6 {
                        class: "footer-title",
                        "Resources"
                    }
                    a {
                        href: links.blog.clone(),
                        class: "block link-hover",
                        "Blog"
                    }
                    a {
                        href: links.pricing.clone(),
                        class: "block link-hover",
                        "Pricing"
                    }
                }
                nav {
                    h6 {
                        class: "footer-title",
                        "Company"
                    }
                    if let Some(about) = links.about.clone() {
                        a {
                            class: "block link-hover",
                            href: about,
                            "About Us"
                        }
                    } else {
                        a {
                            class: "block link-hover",
                            "About Us"
                        }
                    }
                    a {
                        href: links.contact.clone(),
                        class: "block link-hover",
                        "Contact"
                    }
                }
                nav {
                    h6 {
                        class: "footer-title",
                        "Legal"
                    }
                    a {
                        href: links.terms.clone(),
                        class: "block link-hover",
                        "Terms of Use"
                    }
                    a {
                        href: links.privacy.clone(),
                        class: "block link-hover",
                        "Privacy Policy"
                    }
                }
            }
        }
    }
}
