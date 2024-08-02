use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use dioxus::prelude::*;

use crate::layout::Layout;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Doc {
    title: &'static str,
    link: &'static str,
    markdown: &'static str,
}

pub async fn generate() {
    let src = Path::new("docs");
    let dst = Path::new("dist/docs");
    copy_folder(src, dst).unwrap();

    let docs: Vec<Doc> = vec![Doc {
        title: "Why Companies are banning ChatGPT",
        link: "/blog/templates-diffing/",
        markdown: include_str!("../docs/community-edition/introduction.md"),
    }];

    for doc in docs {
        let html = crate::render_with_props(Document, DocumentProps { post: doc });

        let mut file = File::create("dist/docs/community-edition/introduction.html")
            .expect("Unable to create file");
        file.write_all(html.as_bytes())
            .expect("Unable to write to file");
    }
}

#[component]
pub fn Document(post: Doc) -> Element {
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

fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_folder(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
