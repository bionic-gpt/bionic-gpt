use crate::routes::{blog, docs, marketing, product, solutions, SIGN_IN_UP};
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum Section {
    None,
    Home,
    Enterprise,
    Partners,
    McpServers,
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
pub fn Navigation(mobile_menu: Option<Element>, section: Section) -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-50 backdrop-filter backdrop-blur-lg bg-opacity-30",
            div {
                class: "navbar justify-between",

                // Left side: logo + menu
                div {
                    class: "flex items-center gap-4",

                    // Logo
                    a {
                        href: marketing::Index {}.to_string(),
                        span {
                            class: "pl-3 flex flex-row gap-2",
                            strong { "Bionic" }
                        }
                    }

                    // Desktop menu (left aligned)
                    div { class: "hidden lg:flex",
                        ul { class: "menu menu-horizontal px-1 dropdown-content",
                            li {
                                details {
                                    summary {
                                        "Product"
                                    }
                                    ul {
                                        class: "p-2",
                                        li {
                                            a {
                                                href: product::Chat {}.to_string(),
                                                "Chat"
                                            }
                                        }
                                        li {
                                            a {
                                                href: product::Assistants {}.to_string(),
                                                "Assistants"
                                            }
                                        }
                                        li {
                                            a {
                                                href: product::Integrations {}.to_string(),
                                                "Integrations"
                                            }
                                        }
                                        li {
                                            a {
                                                href: product::Automations {}.to_string(),
                                                "Automations"
                                            }
                                        }
                                        li {
                                            a {
                                                href: product::Developers {}.to_string(),
                                                "Developers"
                                            }
                                        }
                                    }
                                }
                            }
                            li {
                                details {
                                    summary {
                                        "Solutions"
                                    }
                                    ul {
                                        class: "w-60 p-2",
                                        li {
                                            a {
                                                href: solutions::Education {}.to_string(),
                                                "Bionic in Education"
                                            }
                                        },
                                        li {
                                            a {
                                                href: solutions::Support {}.to_string(),
                                                "Bionic for Technical Support"
                                            }
                                        }
                                    }
                                }
                            }
                            NavItem {
                                link: marketing::Pricing {}.to_string(),
                                name: "Pricing".to_string(),
                                section: Section::Pricing,
                                current_section: section.clone(),
                            }
                            li {
                                details {
                                    summary {
                                        "Resources"
                                    }
                                    ul {
                                        class: "p-2",
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
                                    }
                                }
                            }
                            NavItem {
                                link: marketing::PartnersPage {}.to_string(),
                                name: "Partners".to_string(),
                                section: Section::Partners,
                                current_section: section.clone(),
                            }
                        }
                    }
                }

                // Right side: GitHub + login + CTA
                div { class: "hidden lg:flex items-center",
                    ul { class: "menu menu-horizontal px-3",
                        li {
                            a {
                                href: "https://github.com/bionic-gpt/bionic-gpt",
                                img {
                                    src: "https://img.shields.io/github/stars/bionic-gpt/bionic-gpt",
                                    alt: "Github"
                                }
                            }
                        }
                        li {
                            a { href: SIGN_IN_UP, "Login" }
                        }
                        NavItem {
                            class: "btn btn-primary btn-sm",
                            link: marketing::Contact {}.to_string(),
                            name: "Book a Call".to_string(),
                            section: Section::Contact,
                            current_section: section.clone(),
                        }
                    }
                }

                // Mobile menu (hamburger)
                div { class: "dropdown lg:hidden dropdown-end",
                    div {
                        tabindex: "0",
                        role: "button",
                        class: "btn btn-ghost",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            fill: "none",
                            class: "h-5 w-5",
                            path {
                                d: "M4 6h16M4 12h8m-8 6h16",
                                stroke_linejoin: "round",
                                stroke_linecap: "round",
                                stroke_width: "2"
                            }
                        }
                    }
                    ul {
                        class: "menu menu-sm dropdown-content mt-3 z-1 p-2 shadow-sm bg-base-100 rounded-box w-52",
                        NavItem {
                            link: marketing::Index {}.to_string(),
                            name: "Home".to_string(),
                            section: Section::Home,
                            current_section: section.clone(),
                        }
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
            }
        }
    }
}
