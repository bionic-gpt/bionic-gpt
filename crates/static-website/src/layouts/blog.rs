use super::layout::Layout;
use crate::{
    components::{extra_footer::ExtraFooter, footer::Footer, navigation::Section},
    generator::{Page, Summary},
};
use dioxus::prelude::*;

#[component]
pub fn BlogPost(post: Page) -> Element {
    let image = if post.image.is_some() {
        post.image.unwrap()
    } else {
        ""
    };
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            image: "{image}",
            section: Section::Blog,
            article {
                class: "mt-24 mx-auto prose lg:prose-xl p-5",
                h1 {
                    "{post.title}"
                }
                div {
                    class: "not-prose flex flex-row mt-8 mb-4",
                    img {
                        width: "44",
                        height: "44",
                        src: post.author_image,
                        alt: "Author"
                    }
                    div {
                        class: "not-prose flex flex-col pl-2",
                        if let Some(author) = post.author {
                            strong {
                                class: "text-base",
                                "{author}"
                            }
                        }
                        small {
                            class: "text-base",
                            "{post.date}"
                        }
                    }
                }
                div {
                    class: "not-prose flex justify-between items-center border-t border-b mb-4",
                    small {
                        class: "not-prose",
                        "Share"
                    }
                    div {
                        class: "not-prose flex flex-row gap-1",
                        a {
                            href: "https://twitter.com/intent/tweet?url={post.permalink()}",
                            img {
                                width: "16",
                                height: "16",
                                src: "/social-sharing/x-twitter.svg"
                            }
                        }
                        a {
                            href: "https://www.linkedin.com/sharing/share-offsite/?url={post.permalink()}",
                            img {
                                width: "16",
                                height: "16",
                                src: "/social-sharing/linkedin.svg"
                            }
                        }
                    }
                }
                img {
                    class: "mb-8 object-cover h-96 w-full",
                    src: "{post.image.unwrap()}",
                    alt: "{post.title}"
                }
                div {
                    dangerous_inner_html: "{content}"
                }
            }
            ExtraFooter {
                title: "You're seconds away from trying us out",
                image: "/landing-page/bionic-console.png",
                cta: "Find out More",
                cta_url: "/"
            }
            Footer {
                extra_class: ""
            }
        }
    }
}

#[component]
pub fn BlogList(summary: Summary) -> Element {
    rsx! {
        Layout {
            title: "Blog",
            description: "Blog",
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "lg:max-w-5xl mx-auto text-center mb-12 mt-24",
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
                                    a {
                                        href: "/{page.folder}",
                                        img {
                                            class: "object-cover h-24 w-full",
                                            src: page.image
                                        }
                                    }
                                    div {
                                        div {
                                            h3 {
                                                "{page.title}"
                                            }
                                            p {
                                                class: "subtitle",
                                                strong {
                                                    "{page.date}"
                                                }
                                            }
                                            p {
                                                a {
                                                    href: "/{page.folder}",
                                                    "Read More..."
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
            Footer {
                extra_class: ""
            }
        }
    }
}
