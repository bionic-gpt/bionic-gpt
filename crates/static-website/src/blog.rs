use crate::layout::Layout;
use crate::summary::Page;
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
