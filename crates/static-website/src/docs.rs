use dioxus::prelude::*;

use crate::layout::Layout;
use crate::summary::Page;

#[component]
pub fn Document(post: Page) -> Element {
    let content = markdown::to_html(post.markdown);
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
