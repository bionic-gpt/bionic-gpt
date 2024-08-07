use super::layout::Layout;
use crate::{
    components::footer::Footer,
    generator::{Page, Summary},
};
use dioxus::prelude::*;
use markdown::{CompileOptions, Options};

#[component]
pub fn BlogPost(post: Page) -> Element {
    let content = markdown::to_html_with_options(
        post.markdown,
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
        Layout {
            title: "{post.title}",
            article {
                class: "mt-12 mx-auto prose lg:prose-xl p-4",
                h1 {
                    "{post.title}"
                }
                img {
                    class: "mb-8 object-cover h-96 w-full",
                    src: "{post.image.unwrap()}"
                }
                div {
                    dangerous_inner_html: "{content}"
                }
            }
            Footer {}
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
                                            class: "object-cover h-24 w-full",
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
            Footer {}
        }
    }
}
