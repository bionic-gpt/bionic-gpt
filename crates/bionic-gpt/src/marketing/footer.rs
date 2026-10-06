use dioxus::prelude::*;
use ssg_whiz::FooterLinks;

#[component]
pub fn Footer(margin_top: Option<String>, links: FooterLinks) -> Element {
    let extra_class = if let Some(extra_class) = margin_top {
        extra_class
    } else {
        "mt-24".to_string()
    };

    rsx! {
        footer {
            class: "{extra_class} bg-neutral text-neutral-content p-10",
            div {
                class: "site-container site-container-content site-gutters-flush flex flex-col justify-between md:flex-row",
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
