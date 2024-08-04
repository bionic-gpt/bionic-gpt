use crate::components::layout::Layout;
use crate::summary::{Page, Summary};
use dioxus::prelude::*;

#[component]
pub fn BlogPost(post: Page) -> Element {
    let content = markdown::to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            article {
                class: "mx-auto prose lg:prose-xl p-4",
                h1 {
                    "{post.title}"
                }
                img {
                    class: "mb-8",
                    width: "768",
                    height: "487",
                    src: "chat-gpt-banned.png"
                }
                div {
                    dangerous_inner_html: "{content}"
                }
            }
        }
    }
}

#[component]
pub fn BlogList(summary: Summary) -> Element {
    rsx! {
        Layout {
            title: "Blog",
            section {
                class: "lg:max-w-5xl mx-auto text-center mb-12 mt-12",
                h1 {
                    class: "text-4xl font-extrabold",
                    "Enterprise Generative AI"
                }
                h2 {
                    class: "text-2xl font-bold",
                    "The Bionic blog explores issues around LLMs in the enterprise"
                }
            }
            section {
                class: "lg:max-w-5xl mx-auto p-4",
                div {
                    div {
                        class: "md:grid grid-cols-2 gap-4",
                        for category in summary.categories {
                            for page in category.pages {
                                div {
                                    class: "border p-4",
                                    div {
                                        img {
                                            src: page.image
                                        }
                                        a {
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
    }
}
