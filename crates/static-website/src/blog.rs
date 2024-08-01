use crate::layout::Layout;
use crate::routes::blog::Index;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;
use dioxus::prelude::*;

pub fn routes() -> Router {
    Router::new().typed_get(index)
}

pub async fn index(Index { slug }: Index) -> Html<String> {
    let html = crate::render_with_props(Blog, BlogProps { slug });

    Html(html)
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BlogPost {
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    markdown: &'static str,
}

pub const POST_TEMPLATE: BlogPost = BlogPost {
    date: "Dec 11, 2022",
    title: "Why Companies are banning ChatGPT",
    description:
        "Using a new technique called subtree memoization, Dioxus is now almost as fast as SolidJS.",
    link: "/blog/templates-diffing/",
    markdown: include_str!("../posts/banning-chat-gpt/index.md"),
};

#[component]
pub fn Blog(slug: String) -> Element {
    rsx! {
        BlogPost {
            post: POST_TEMPLATE
        }
    }
}

#[component]
pub fn BlogPost(post: BlogPost) -> Element {
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
