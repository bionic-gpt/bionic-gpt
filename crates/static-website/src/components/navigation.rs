use crate::routes::{blog, contact, docs, marketing, pricing, SIGN_IN_UP};
use dioxus::prelude::*;

#[component]
pub fn Navigation() -> Element {
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
                            li {}
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
                            a { href: pricing::Index {}.to_string(), "Pricing" }
                        }
                        li {
                            a { href: docs::Index {}.to_string(), "Documentation" }
                        }
                        li {
                            a { href: blog::Index {}.to_string(), "Blog" }
                        }
                        li {
                            a { href: contact::Index {}.to_string(), "Contact Us" }
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
                        li {
                            label { class: "swap swap-rotate",
                                input {
                                    "data-theme-toggle": "false",
                                    value: "synthwave",
                                    r#type: "checkbox",
                                    class: "theme-controller"
                                }
                                svg {
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "viewBox": "0 0 24 24",
                                    class: "swap-off fill-current w-5 h-5",
                                    path { "d": "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                                }
                                svg {
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "viewBox": "0 0 24 24",
                                    class: "swap-on fill-current w-5 h-5",
                                    path { "d": "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
