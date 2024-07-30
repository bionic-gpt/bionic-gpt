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
    let content = markdown::to_html(POST_TEMPLATE.markdown);
    rsx! {
        div {
            class: "prose",
            dangerous_inner_html: "{content}"
        }
    }
}
