use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use dioxus::prelude::*;

use crate::components::navigation::Section;
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

fn write_page(dest_folder: &str, html: String) -> io::Result<()> {
    fs::create_dir_all(dest_folder)?;
    let mut file = File::create(format!("{}/index.html", dest_folder))?;
    file.write_all(html.as_bytes())
}

pub async fn generate_product() {
    let html = pages::product::assistants::page();
    write_page("dist/product/assistants", html).expect("Unable to write page");

    let html = pages::product::automations::page();
    write_page("dist/product/automations", html).expect("Unable to write page");

    let html = pages::product::chat::page();
    write_page("dist/product/chat", html).expect("Unable to write page");

    let html = pages::product::developers::page();
    write_page("dist/product/developers", html).expect("Unable to write page");

    let html = pages::product::integrations::page();
    write_page("dist/product/integrations", html).expect("Unable to write page");
}

pub async fn generate_solutions() {
    let html = pages::solutions::education::page();
    write_page("dist/solutions/education", html).expect("Unable to write page");

    let html = pages::solutions::support::page();
    write_page("dist/solutions/support", html).expect("Unable to write page");
}

pub async fn generate_marketing() {
    let html = pages::pricing::pricing();
    write_page("dist/pricing", html).expect("Unable to write page");

    let html = pages::partners::partners_page();
    write_page("dist/partners", html).expect("Unable to write page");

    let html = pages::contact::contact_page();
    write_page("dist/contact", html).expect("Unable to write page");

    let html = pages::home::home_page();
    write_page("dist", html).expect("Unable to write page");
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
            write_page(&format!("dist/{}", page.folder), html).expect("Unable to write page");
        }
    }
}

pub fn generate_docs(summary: Summary, section: Section) {
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
                    current_section: section.clone(),
                }
            };

            let html = crate::render(page_ele);
            write_page(&format!("dist/{}", page.folder), html).expect("Unable to write page");
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
            write_page(&format!("dist/{}", page.folder), html).expect("Unable to write page");
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
    write_page("dist/blog", html).expect("Unable to write page");
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
