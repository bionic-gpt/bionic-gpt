use dioxus::prelude::*;

use super::layout::Layout;
use crate::{
    components::navigation::Section,
    generator::{Category, Page, Summary},
};

#[component]
pub fn Document(summary: Summary, category: Category, doc: Page) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            description: "{doc.description}",
            section: Section::Docs,
            mobile_menu: rsx! (MobileMenu {
                summary: summary.clone()
            }),
            main {
                class: "flex-1",

                div {
                    class: "flex flex-row relative",
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
            class: "fixed z-40 lg:z-auto w-0 -left-full lg:w-[420px] !lg:left-0 lg:sticky h-[calc(100vh-68px)] top-2 bottom-0 flex flex-col ml-0 border-r lg:overflow-y-auto",
            nav {
                class: "pt-12 p-5",
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
                                    "hx-boost": "true",
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
    let content = crate::markdown::markdown_to_html(doc.markdown);
    rsx! {
        section {
            class: "p-5 pt-12 w-full h-[calc(100vh-68px)] lg:overflow-y-auto",
            div {
                class: "mb-12",
                article {
                    class: "mx-auto prose",
                    div {
                        dangerous_inner_html: "{content}"
                    }
                }
            }
        }
    }
}
