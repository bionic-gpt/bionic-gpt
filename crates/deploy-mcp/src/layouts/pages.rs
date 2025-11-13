use super::layout::Layout;
use crate::{
    components::{
        extra_footer::{ExtraFooter, EXTRA_FOOTER_TITLE},
        footer::Footer,
        navigation::Section,
    },
    generator::Page,
};
use dioxus::prelude::*;

#[component]
pub fn MarkdownPage(post: Page) -> Element {
    let content = crate::markdown::markdown_to_html(post.markdown);
    rsx! {
        Layout {
            title: "{post.title}",
            description: "{post.description}",
            url: Some(post.permalink()),
            section: Section::None,
            article {
                class: "mx-auto max-w-2xl p-5",
                div {
                    class: "prose",
                    dangerous_inner_html: "{content}"
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
