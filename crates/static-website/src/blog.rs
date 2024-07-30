use crate::footer::Footer;
use crate::navigation::Navigation;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) struct BlogPost {
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    markdown: &'static str,
}

pub(crate) const POST_TEMPLATE: BlogPost = BlogPost {
    date: "Dec 11, 2022",
    title: "Making Dioxus (almost) as fast as SolidJS",
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
        Navigation {

        }
        article {
            class: "mx-auto prose lg:prose-xl p-4",
            dangerous_inner_html: "{content}",
            h1 {
                "{post.title}"
            }
        }
        Footer {

        }
    }
}
