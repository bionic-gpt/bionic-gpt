use crate::marketing::{
    footer::Footer,
    layout::{PageContainer, PageWidth},
};
use crate::ui_links::footer_links;
use dioxus::prelude::*;
use ssg_whiz::layouts::layout::Layout;
use ssg_whiz::Section;

pub fn go_bionic_page() -> String {
    let page = rsx! {
        Layout {
            title: "Go Bionic",
            description: "Deploy Bionic yourself or get help deploying it on your own infrastructure.",
            mobile_menu: None,
            section: Section::None,
            PageContainer { width: PageWidth::Content, class: Some("mt-16 md:mt-24".to_string()),
                section {
                    class: "mx-auto max-w-3xl text-center",
                    h1 {
                        "Go Bionic"
                    }
                    p {
                        class: "mt-6 text-lg leading-8 opacity-80",
                        "Bionic is open source, licensed under "
                        strong { "Apache 2.0" }
                        ", and can be deployed on-premise or in your private infrastructure, giving your organisation control over its data and models."
                    }
                }
                section {
                    class: "mt-12 grid gap-6 md:grid-cols-2",
                    div {
                        class: "card card-border bg-base-100",
                        div {
                            class: "card-body",
                            h3 { class: "card-title", "Deploy it yourself" }
                            p {
                                class: "mb-3",
                                "Deploy and extend Bionic yourself. The open-source platform includes:"
                            }
                            ul {
                                class: "list-disc space-y-1 pl-5",
                                li { "Private and local model support" }
                                li { "Knowledge search and RAG" }
                                li { "Integrations and an agentic runtime" }
                                li { "Identity, permissions and audit" }
                            }
                            div {
                                class: "mt-4 flex flex-wrap gap-3",
                                a {
                                    class: "btn btn-primary",
                                    href: "/docs/running-locally/docker-compose/",
                                    "Get started"
                                }
                                a {
                                    class: "btn btn-secondary btn-outline",
                                    href: "https://github.com/bionic-gpt/bionic-gpt",
                                    "View on GitHub"
                                }
                            }
                        }
                    }
                    div {
                        class: "card card-border bg-base-100",
                        div {
                            class: "card-body",
                            h3 { class: "card-title", "Get help with deployment" }
                            p {
                                class: "grow",
                                "We can help deploy Bionic in your environment, configure SSO and models, connect an initial integration, and help your team get a first workflow running."
                            }
                            div {
                                class: "mt-4",
                                a {
                                    class: "btn btn-primary",
                                    href: crate::routes::marketing::Contact {}.to_string(),
                                    "Contact us"
                                }
                            }
                        }
                    }
                }
                section {
                    class: "mt-6 w-full",
                    div {
                        class: "card card-border bg-base-100",
                        div {
                            class: "card-body",
                            h3 { class: "card-title", "Try the demo" }
                            p {
                                class: "grow",
                                "This is a trial of Bionic using a relatively small model, so you can explore the experience before deciding how to deploy."
                            }
                            div {
                                class: "mt-4",
                                a {
                                    class: "btn btn-secondary",
                                    href: crate::routes::SIGN_IN_UP,
                                    "Try the demo"
                                }
                            }
                        }
                    }
                }
            }
            Footer {
                links: footer_links()
            }
        }
    };

    crate::render(page)
}
