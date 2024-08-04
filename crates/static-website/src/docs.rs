use dioxus::prelude::*;

use crate::components::layout::Layout;
use crate::summary::{Category, Page, Summary};

#[component]
pub fn Document(summary: Summary, category: Category, doc: Page) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            div {
                class: "w-full text-sm dark:bg-ideblack",
                min_height: "100vh",

                // Flex centered, every column grows to split into 3
                div { class: "flex flex-row justify-center dark:text-[#dee2e6] font-light",
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
fn LeftNav(summary: Summary) -> Element {
    // We use this to remove the spacing between "Introduction" and "Getting Started"
    // TODO: Make this depend on if the chapter has any links.
    //let mut keep_bottom_spacing = false;

    rsx! {
        // Create a flex grow container, and then right-align its contents so it's squahed against the center
        div { class: "overflow-y-auto sticky docs-links pt-12 flex flex-row justify-end",
            nav {
                class: "bg-white dark:bg-ideblack lg:bg-inherit pl-6 pb-32 z-20 text-base lg:block top-28 lg:-ml-3.5 pr-2 w-[calc(100%-1rem)] md:w-60 lg:text-[14px] text-navy content-startleading-5 ",
                //class: if SHOW_SIDEBAR() { "min-w-full" } else { "hidden" },
                //for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                //    SidebarSection { chapter, keep_bottom_spacing }
                //   {keep_bottom_spacing = true}
                //}
                for category in &summary.categories {
                    p {
                        class: "font-semibold mb-2 mt-6",
                        "{category.name}"
                    }
                    ul {
                        for page in &category.pages {
                            li {
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
    let content = markdown::to_html(doc.markdown);
    rsx! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack container pb-12 max-w-screen-sm mx-2 lg:mx-24 pt-12 grow",
            div {
                class: "-py-8",
                //class: if HIGHLIGHT_DOCS_LAYOUT() { "border border-green-600 rounded-md" },
                div {
                    class: "flex w-full mb-20 flex-wrap list-none",
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
}
