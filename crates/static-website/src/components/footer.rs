use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer bg-neutral text-neutral-content p-10",
            nav {
                h6 {
                    class: "footer-title",
                    "Resources"
                }
                a {
                    href: crate::routes::blog::Index {}.to_string(),
                    class: "link link-hover",
                    "Blog"
                }
                a {
                    href: crate::routes::marketing::Pricing {}.to_string(),
                    class: "link link-hover",
                    "Pricing"
                }
            }
            nav {
                h6 {
                    class: "footer-title",
                    "Company"
                }
                a {
                    class: "link link-hover",
                    "About Us"
                }
                a {
                    href: crate::routes::marketing::Contact {}.to_string(),
                    class: "link link-hover",
                    "Contact"
                }
            }
            nav {
                h6 {
                    class: "footer-title",
                    "Legal"
                }
                a {
                    href: crate::routes::marketing::Terms {}.to_string(),
                    class: "link link-hover",
                    "Terms of Use"
                }
                a {
                    href: crate::routes::marketing::Privacy {}.to_string(),
                    class: "link link-hover",
                    "Privacy Policy"
                }
            }
        }
    }
}
