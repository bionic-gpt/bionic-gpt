use dioxus::prelude::*;

use super::layout::Layout;
use crate::{
    components::navigation::Section,
    generator::{Category, Page, Summary},
};

#[component]
pub fn Document(
    summary: Summary,
    category: Category,
    doc: Page,
    current_section: Section,
) -> Element {
    rsx! {
        Layout {
            title: "{doc.title}",
            description: "{doc.description}",
            section: current_section,
            mobile_menu: rsx! (MobileMenu {
                summary: summary.clone()
            }),
            main {
                class: "flex-1",

                div {
                    class: "flex flex-row relative",
                    LeftNav {
                        summary: summary.clone(),
                        active_folder: doc.folder,
                        scroll_key: summary.source_folder,
                    }
                    Content {
                        doc
                    }
                }
                // Preserve sidebar scroll between navigations so the left nav
                // stays at the same position after clicking a link.
                script {
                    dangerous_inner_html: format!(r#"
                        (function() {{
                            const nav = document.querySelector('[data-scroll-key="{key}"]');
                            if (!nav) return;
                            const storageKey = "left-nav-scroll-{key}";
                            const saved = sessionStorage.getItem(storageKey);
                            if (saved) {{
                                nav.scrollTop = parseInt(saved, 10) || 0;
                            }}
                            nav.addEventListener("scroll", function() {{
                                sessionStorage.setItem(storageKey, nav.scrollTop.toString());
                            }}, {{ passive: true }});
                        }})();
                    "#, key = summary.source_folder)
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
fn LeftNav(summary: Summary, active_folder: &'static str, scroll_key: &'static str) -> Element {
    rsx! {
        div {
            class: "fixed z-40 lg:z-auto w-0 -left-full lg:w-[420px] !lg:left-0 lg:sticky h-[calc(100vh-108px)] top-2 bottom-0 flex flex-col ml-0 border-r lg:overflow-y-auto",
            "data-scroll-key": scroll_key,
            nav {
                class: "pt-12 p-5",
                for category in &summary.categories {
                    p {
                        class: format!(
                            "font-semibold mb-2 {}",
                            if category.name.contains("Coming Soon") {
                                "opacity-60"
                            } else {
                                ""
                            }
                        ),
                        "{category.name}"
                    }
                    ul {
                        class: "mb-6",
                        for page in &category.pages {
                            li {
                                class: "mb-2",
                                a {
                                    class: format!(
                                        "rounded-md hover:text-sky-500 dark:hover:text-sky-400 {} {}",
                                        if page.folder == active_folder && !category.name.contains("Coming Soon") {
                                            "text-primary font-semibold border-b-2 border-primary pb-[2px]"
                                        } else {
                                            ""
                                        },
                                        if category.name.contains("Coming Soon") {
                                            "opacity-50 pointer-events-none cursor-not-allowed"
                                        } else {
                                            ""
                                        }
                                    ),
                                    href: "/{page.folder}",
                                    "hx-boost": if category.name.contains("Coming Soon") { "false" } else { "true" },
                                    tabindex: if category.name.contains("Coming Soon") { "-1" } else { "0" },
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
            class: "p-5 pt-12 w-full h-[calc(100vh-108px)] lg:overflow-y-auto",
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
