use dioxus::prelude::*;
use markdown::{CompileOptions, Options};

use super::layout::Layout;
use crate::generator::{Category, Page, Summary};

#[component]
pub fn Document(summary: Summary, category: Category, doc: Page) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            description: "{doc.description}",
            mobile_menu: rsx! (MobileMenu {
                summary: summary.clone()
            }),
            div {
                class: "w-full text-sm dark:bg-ideblack",

                div {
                    class: "flex flex-row",
                    LeftNav {
                        summary
                    }
                    Content {
                        doc
                    }
                }
            }
        }
    }
}

#[component]
fn MobileMenu(summary: Summary) -> Element {
    rsx! {
        for category in &summary.categories {
            ul {
                for page in &category.pages {
                    li {
                        a {
                            href: "/{page.folder}",
                            "{page.title}",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LeftNav(summary: Summary) -> Element {
    rsx! {
        div {
            class: "h-[calc(100vh-68px)] hidden lg:flex",
            nav {
                class: "h-[calc(100vh-86px)] overflow-scroll p-3",
                for category in &summary.categories {
                    p {
                        class: "font-semibold mb-2",
                        "{category.name}"
                    }
                    ul {
                        class: "mb-6",
                        for page in &category.pages {
                            li {
                                class: "mb-2",
                                a {
                                    class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400",
                                    href: "/{page.folder}",
                                    "{page.title}"
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn Content(doc: Page) -> Element {
    let content = markdown::to_html_with_options(
        doc.markdown,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..CompileOptions::default()
            },
            ..Options::default()
        },
    )
    .expect("Couldn't generate markdown");
    rsx! {
        section {
            class: "grow h-[calc(100vh-68px)] p-2",
            div {
                class: "bg-slate-100 rounded border border-slate-300 flex justify-center h-[calc(100vh-86px)] overflow-scroll",
                article {
                    class: "prose",
                    div {
                        dangerous_inner_html: "{content}"
                    }
                }
            }
        }
    }
}
