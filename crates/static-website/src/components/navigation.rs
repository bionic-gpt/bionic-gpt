use crate::routes::{blog, docs, marketing, SIGN_IN_UP};
use dioxus::prelude::*;

#[component]
pub fn Navigation(mobile_menu: Element) -> Element {
    rsx! {
        header {
            div {
                class: "navbar justify-between bg-base-100",
                div {
                    div { class: "dropdown lg:hidden",
                        div {
                            tabindex: "0",
                            role: "button",
                            class: "btn btn-ghost",
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "stroke": "currentColor",
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                class: "h-5 w-5",
                                path {
                                    "d": "M4 6h16M4 12h8m-8 6h16",
                                    "stroke-linejoin": "round",
                                    "stroke-linecap": "round",
                                    "stroke-width": "2"
                                }
                            }
                        }
                        ul {
                            class: "menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52",
                            li {
                                a {
                                    href: blog::Index {}.to_string(),
                                    "Blog"
                                }
                            }
                            li {
                                a {
                                    href: docs::Index {}.to_string(),
                                    "Documentation"
                                }
                            }
                            li {
                                a { href: SIGN_IN_UP, "Sign Up" }
                            }
                            li {
                                a { href: SIGN_IN_UP, "Sign In" }
                            }
                            {mobile_menu}
                        }
                    }
                    a {
                        href: marketing::Index {}.to_string(),
                        span {
                            class: "flex flex-row gap-4",
                            img {
                                alt: "Logo",
                                width: "22",
                                height: "22",
                                src: "/bionic-logo.svg"
                            }
                            "Bionic"
                        }
                    }
                }
                div { class: "navbar-center hidden lg:flex",
                    ul { class: "menu menu-horizontal px-1",
                        li {
                            a { href: marketing::Pricing {}.to_string(), "Pricing" }
                        }
                        li {
                            a { href: docs::Index {}.to_string(), "Documentation" }
                        }
                        li {
                            a { href: blog::Index {}.to_string(), "Blog" }
                        }
                        li {
                            a { href: marketing::PartnersPage {}.to_string(), "Partners" }
                        }
                        li {
                            a { href: marketing::ServicesPage {}.to_string(), "Services" }
                        }
                        li {
                            a { href: marketing::Contact {}.to_string(), "Contact Us" }
                        }
                    }
                }
                div { class: "hidden lg:flex",
                    ul { class: "menu menu-horizontal px-1",
                        li {
                            a {
                                href: "https://github.com/bionic-gpt/bionic-gpt",
                                img { src: "https://img.shields.io/github/stars/bionic-gpt/bionic-gpt" }
                            }
                        }
                        li {
                            a { href: SIGN_IN_UP, "Sign Up" }
                        }
                        li {
                            a { href: SIGN_IN_UP, "Sign In" }
                        }
                    }
                }
            }
        }
    }
}
