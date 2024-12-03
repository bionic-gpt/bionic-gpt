use crate::routes::{blog, docs, marketing, SIGN_IN_UP};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Section {
    None,
    Home,
    Enterprise,
    Partners,
    Pricing,
    Blog,
    Docs,
    Contact,
}

#[component]
pub fn NavItem(
    link: String,
    name: String,
    section: Section,
    current_section: Section,
    class: Option<String>,
) -> Element {
    let mut added_class = "";
    if section == current_section {
        added_class = "underline";
    }
    let class = if let Some(class) = class {
        class
    } else {
        "".to_string()
    };
    let class = format!("{} {}", class, added_class);
    rsx!(
        li {
            a {
                class: format!("{}", class),
                "hx-boost": "true",
                href: link,
                "{name}"
            }
        }
    )
}

#[component]
pub fn Navigation(mobile_menu: Element, section: Section) -> Element {
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
                            NavItem {
                                link: marketing::Pricing {}.to_string(),
                                name: "Pricing".to_string(),
                                section: Section::Pricing,
                                current_section: section.clone(),
                            }
                            NavItem {
                                link: blog::Index {}.to_string(),
                                name: "Blog".to_string(),
                                section: Section::Blog,
                                current_section: section.clone(),
                            }
                            NavItem {
                                link: docs::Index {}.to_string(),
                                name: "Documentation".to_string(),
                                section: Section::Docs,
                                current_section: section.clone(),
                            }
                            NavItem {
                                link: marketing::PartnersPage {}.to_string(),
                                name: "Partners".to_string(),
                                section: Section::Partners,
                                current_section: section.clone(),
                            }
                            {mobile_menu}
                        }
                    }
                    a {
                        href: marketing::Index {}.to_string(),
                        span {
                            class: "pl-5 flex flex-row gap-2",
                            img {
                                alt: "Logo",
                                width: "22",
                                height: "22",
                                src: "/logo.svg"
                            }
                            strong {
                                "Bionic-GPT"
                            }
                        }
                    }
                }
                div { class: "navbar-center hidden lg:flex",
                    ul { class: "menu menu-horizontal px-1",
                        NavItem {
                            link: marketing::Pricing {}.to_string(),
                            name: "Pricing".to_string(),
                            section: Section::Pricing,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: blog::Index {}.to_string(),
                            name: "Blog".to_string(),
                            section: Section::Blog,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: docs::Index {}.to_string(),
                            name: "Documentation".to_string(),
                            section: Section::Docs,
                            current_section: section.clone(),
                        }
                        NavItem {
                            link: marketing::PartnersPage {}.to_string(),
                            name: "Partners".to_string(),
                            section: Section::Partners,
                            current_section: section.clone(),
                        }
                    }
                }
                div { class: "hidden lg:flex",
                    ul { class: "menu menu-horizontal px-5",
                        li {
                            a {
                                href: "https://github.com/bionic-gpt/bionic-gpt",
                                img { src: "https://img.shields.io/github/stars/bionic-gpt/bionic-gpt" }
                            }
                        }
                        li {
                            a { href: SIGN_IN_UP, "Sign In/Up" }
                        }
                        NavItem {
                            class: "btn btn-primary btn-sm",
                            link: marketing::Contact {}.to_string(),
                            name: "Book a Demo".to_string(),
                            section: Section::Contact,
                            current_section: section.clone(),
                        }
                    }
                }
            }
        }
    }
}
