use super::layout::Layout;
use crate::summary::Page;
use dioxus::prelude::*;
use markdown::{CompileOptions, Options};

#[component]
pub fn MarkdownPage(post: Page) -> Element {
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
                class: "mx-auto prose lg:prose-xl p-4",
                h1 {
                    "{post.title}"
                }
                div {
                    dangerous_inner_html: "{content}"
                }
            }
        }
    }
}
