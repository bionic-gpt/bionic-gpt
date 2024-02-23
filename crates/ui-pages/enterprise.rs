#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use assets::files::avatar_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use dioxus::prelude::*;

#[inline_props]
pub fn Page(cx: Scope, team_id: i32, rbac: Rbac) -> Element {
    cx.render(rsx! {
        Layout {
            section_class: "normal",
            selected_item: SideBar::None,
            title: "Your Installation",
            team_id: *team_id,
            rbac: rbac,
            header: cx.render(rsx!(
                h3 { "Your Installation" }
            )),
            BlankSlate {
                heading: "Your Installation",
                visual: avatar_svg.name,
                description: "Here you can see how you are progressing towards a Bionic-GPT full installation.",
            }

            if std::env::var("COMMUNITY_EDITION").is_ok() {
                cx.render(rsx!(
                    Box {
                        BoxHeader {
                            title: "You are using Community Edition"
                        }
                        BoxBody {
                            Alert {
                                alert_color: AlertColor::Error,
                                "Do not deploy community edition into production.
                                It is meant for demonstration purposes only."
                            }
                            div {
                                class: "prose max-w-none",
                                p {
                                    "Follow our guide to ",
                                    a {
                                        href: "https://bionic-gpt.com/docs/enterprise-edition",
                                        "install enterprise edition."
                                    }
                                    " You will then unlock the following features."
                                }
                                ul {
                                    li {
                                        "High availability and robustness."
                                    }
                                    li {
                                        "A secure solution with all security best practices."
                                    }
                                    li {
                                        "Document Pipelines"
                                    }
                                    li {
                                        "Image generation"
                                    }
                                }
                            }
                        }
                    }
                ))
            } else {
                cx.render(rsx!(
                    Box {
                        BoxHeader {
                            title: "You are using Enterprise Edition"
                        }
                        BoxBody {
                            Accordian {
                                title: "Register your installation and unlock more features",
                                name: "edition",
                                checked: true,
                                div {
                                    class: "prose max-w-none",
                                    p {
                                        a {
                                            href: "https://bionic-gpt.com/contact/",
                                            "Contact us"
                                        }
                                        " to get an unlock code and you will enable the Document pipelines feature"
                                    }
                                }
                                form {
                                    class: "mt-5",
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            label: "Registration Key",
                                            help_text: "Add the registration key you received by email",
                                            name: "registration_key"
                                        }
                                    }
                                }
                            }
                            Accordian {
                                title: "Licence your installation",
                                name: "edition",
                                div {
                                    class: "prose max-w-none",
                                    p {
                                        a {
                                            href: "https://bionic-gpt.com/contact/",
                                            "Contact us"
                                        }
                                        " to get a licence key and enable the following"
                                    }
                                    h4 {
                                        "Support"
                                    }
                                    ul {
                                        li {
                                            "Support and priority bug fixes"
                                        }
                                        li {
                                            "Help with installation and discovery"
                                        }
                                        li {
                                            "Twice yearly help upgarding to LTS versions"
                                        }
                                        li {
                                            "Security hardening and notification of security updates"
                                        }
                                    }
                                    h4 {
                                        "More Features"
                                    }
                                    ul {
                                        li {
                                            "Image generation using stable diffusion"
                                        }
                                        li {
                                            "Guardrails"
                                        }
                                    }
                                }
                                form {
                                    class: "mt-5",
                                    div {
                                        class: "flex flex-col",
                                        Input {
                                            label: "Registration Key",
                                            help_text: "Add the registration key you received by email",
                                            name: "registration_key"
                                        }
                                    }
                                }
                            }
                        }
                    }
                ))
            }
        }
    })
}

pub fn index(props: PageProps) -> String {
    crate::render(VirtualDom::new_with_props(Page, props))
}
