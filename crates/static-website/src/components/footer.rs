use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer bg-neutral text-neutral-content p-10",
            nav {
                h6 {
                    class: "footer-title",
                    "Services"
                }
                a {
                    class: "link link-hover",
                    "Branding"
                }
                a {
                    class: "link link-hover",
                    "Design"
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
                    class: "link link-hover",
                    "Terms of Use"
                }
                a {
                    class: "link link-hover",
                    "Privacy Policy"
                }
            }
        }
    }
}
