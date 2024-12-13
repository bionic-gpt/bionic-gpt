use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use dioxus::prelude::*;

use crate::layouts::blog::{BlogList, BlogPost};
use crate::layouts::docs::Document;
use crate::layouts::pages::MarkdownPage;
use crate::pages;

#[derive(PartialEq, Eq, Clone)]
pub struct Summary {
    pub source_folder: &'static str,
    pub categories: Vec<Category>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Category {
    pub name: String,
    pub pages: Vec<Page>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Page {
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub folder: &'static str,
    pub markdown: &'static str,
    pub image: Option<&'static str>,
    pub author: Option<&'static str>,
    pub author_image: Option<&'static str>,
}

impl Page {
    pub fn permalink(&self) -> String {
        format!("https://bionic-gpt.com/{}", self.folder)
    }
}

pub async fn generate_marketing() {
    let html = pages::pricing::pricing();

    fs::create_dir_all("dist/pricing").expect("Couyldn't create folder");
    let mut file = File::create("dist/pricing/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = pages::partners::partners_page();

    fs::create_dir_all("dist/partners").expect("Couyldn't create folder");
    let mut file = File::create("dist/partners/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = pages::contact::contact_page();

    fs::create_dir_all("dist/contact").expect("Couyldn't create folder");
    let mut file = File::create("dist/contact/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = pages::home::home_page();

    let mut file = File::create("dist/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub fn generate(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();

    for category in summary.categories {
        for page in category.pages {
            let page_ele = rsx! {
                BlogPost {
                    post: page
                }
            };

            let html = crate::render(page_ele);
            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub fn generate_docs(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                Document {
                    summary: summary.clone(),
                    category: category.clone(),
                    doc: *page,
                }
            };

            let html = crate::render(page_ele);
            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub async fn generate_pages(summary: Summary) {
    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                MarkdownPage {
                    post: *page
                }
            };
            let html = crate::render(page_ele);

            let file = format!("dist/{}", page.folder);

            fs::create_dir_all(&file).expect("Could not create directory");

            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub async fn generate_blog_list(summary: Summary) {
    let page_ele = rsx! {
        BlogList {
            summary
        }
    };
    let html = crate::render(page_ele);

    let mut file = File::create("dist/blog/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}

pub fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
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
