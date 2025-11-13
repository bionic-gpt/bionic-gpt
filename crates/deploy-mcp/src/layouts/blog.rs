use super::layout::Layout;
use crate::{
    components::{
        extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
        footer::Footer,
        navigation::Section,
    },
    generator::{Page, Summary},
};
use dioxus::prelude::*;

#[component]
pub fn BlogPost(post: Page) -> Element {
    let image = post.image.unwrap_or_default().to_string();
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(post.permalink()),
            image: if image.is_empty() { None } else { Some(image.clone()) },
            section: Section::Blog,
            article {
                class: "mt-24 mx-auto prose lg:prose-xl p-5",
                h1 { "{post.title}" }
                div {
                    class: "not-prose flex flex-row mt-8 mb-4",
                    if let Some(author_image) = post.author_image {
                        img {
                            width: "44",
                            height: "44",
                            src: author_image,
                            alt: "Author",
                        }
                    }
                    div {
                        class: "not-prose flex flex-col pl-2",
                        if let Some(author) = post.author {
                            strong { class: "text-base", "{author}" }
                        }
                        small { class: "text-base", "{post.date}" }
                    }
                }
                div {
                    class: "not-prose flex justify-between items-center border-t border-b mb-4",
                    small { class: "not-prose", "Share" }
                    div {
                        class: "not-prose flex flex-row gap-1",
                        a {
                            href: "https://twitter.com/intent/tweet?url={post.permalink()}",
                            img { width: "16", height: "16", src: "/social-sharing/x-twitter.svg" }
                        }
                        a {
                            href: "https://www.linkedin.com/sharing/share-offsite/?url={post.permalink()}",
                            img { width: "16", height: "16", src: "/social-sharing/linkedin.svg" }
                        }
                    }
                }
                if !image.is_empty() {
                    img {
                        class: "mb-8 object-cover h-96 w-full",
                        src: "{image}",
                        alt: "{post.title}"
                    }
                }
                div { dangerous_inner_html: "{content}" }
            }
            ExtraFooter {
                title: EXTRA_FOOTER_TITLE.to_string(),
                image: "/docs/mcp-servers.png",
                cta: "Get Started",
                cta_url: crate::routes::marketing::Index {}.to_string(),
            }
            Footer { margin_top: "mt-0" }
        }
    }
}

#[component]
pub fn BlogList(summary: Summary) -> Element {
    rsx! {
        Layout {
            title: "Deploy Blog",
            description: "Stories, experiments, and lessons from the Deploy team.",
            url: Some("https://deploy.run/blog".to_string()),
            mobile_menu: None,
            section: Section::Blog,
            section {
                class: "lg:max-w-5xl mx-auto text-center mb-12 mt-24",
                h1 { class: "text-4xl font-extrabold", "Deploying AI with confidence" }
                h2 {
                    class: "text-2xl font-bold",
                    "Insights from the Deploy product and engineering teams."
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
                                        if let Some(image) = page.image {
                                            img { class: "object-cover h-48 w-full", src: image }
                                        }
                                    }
                                    div {
                                        h3 { "{page.title}" }
                                        p { class: "subtitle", strong { "{page.date}" } }
                                        p { a { href: "/{page.folder}", "Read More..." } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ExtraFooter {
                title: EXTRA_FOOTER_TITLE.to_string(),
                image: "/docs/mcp-servers.png",
                cta: "Get Started",
                cta_url: crate::routes::marketing::Index {}.to_string(),
            }
            Footer { margin_top: "mt-0" }
        }
    }
}
