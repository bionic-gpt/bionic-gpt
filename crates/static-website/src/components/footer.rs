use dioxus::prelude::*;

#[component]
pub fn Footer(margin_top: Option<String>) -> Element {
    let extra_class = if let Some(extra_class) = margin_top {
        extra_class
    } else {
        "mt-24".to_string()
    };
    rsx! {
        footer {
            class: "{extra_class} bg-neutral text-neutral-content p-10",
            div {
                class: "mx-auto lg:max-w-5xl flex flex-row justify-between",
                nav {
                    h6 {
                        class: "footer-title",
                        "Resources"
                    }
                    a {
                        href: crate::routes::blog::Index {}.to_string(),
                        class: "block link-hover",
                        "Blog"
                    }
                    a {
                        href: crate::routes::marketing::Pricing {}.to_string(),
                        class: "block link-hover",
                        "Pricing"
                    }
                }
                nav {
                    h6 {
                        class: "footer-title",
                        "Company"
                    }
                    a {
                        class: "block link-hover",
                        "About Us"
                    }
                    a {
                        href: crate::routes::marketing::Contact {}.to_string(),
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
                        href: crate::routes::marketing::Terms {}.to_string(),
                        class: "block link-hover",
                        "Terms of Use"
                    }
                    a {
                        href: crate::routes::marketing::Privacy {}.to_string(),
                        class: "block link-hover",
                        "Privacy Policy"
                    }
                }
            }
        }
    }
}
