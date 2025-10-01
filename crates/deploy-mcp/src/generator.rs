use std::fs::{self, File};
use std::io::{self, Write};
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
        format!("https://deploy.run/{}", self.folder)
    }
}

fn write_page(dest_folder: &str, html: String) -> io::Result<()> {
    fs::create_dir_all(dest_folder)?;
    let mut file = File::create(format!("{}/index.html", dest_folder))?;
    file.write_all(html.as_bytes())
}

pub fn generate_marketing() {
    write_page("dist", pages::home::home_page()).expect("Unable to write home page");
    write_page("dist/pricing", pages::pricing::pricing_page())
        .expect("Unable to write pricing page");
    write_page("dist/contact", pages::contact::contact_page())
        .expect("Unable to write contact page");
    write_page("dist/enterprise", pages::enterprise::enterprise_page())
        .expect("Unable to write enterprise page");
}

pub fn generate_mcp_servers() {
    let integrations = pages::mcp_servers::load_integration_specs();

    write_page(
        "dist/mcp-servers",
        pages::mcp_servers::index_page(&integrations),
    )
    .expect("Unable to write MCP servers page");

    for integration in integrations {
        let folder = format!("dist/{}", integration.folder_name());
        write_page(&folder, pages::mcp_servers::detail_page(&integration))
            .expect("Unable to write MCP server detail page");
    }
}

pub fn generate_docs(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).expect("Unable to copy docs assets");

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
            write_page(&format!("dist/{}", page.folder), html).expect("Unable to write doc page");
        }
    }
}

pub fn generate_pages(summary: Summary) {
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
pub fn generate_blog_posts(summary: Summary) {
    let src = format!("content/{}", summary.source_folder);
    let src = Path::new(&src);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).expect("Unable to copy blog assets");

    for category in &summary.categories {
        for page in &category.pages {
            let page_ele = rsx! {
                BlogPost { post: *page }
            };
            let html = crate::render(page_ele);
            write_page(&format!("dist/{}", page.folder), html).expect("Unable to write blog page");
        }
    }

    let list_ele = rsx! {
        BlogList { summary: summary.clone() }
    };
    let html = crate::render(list_ele);
    write_page("dist/blog", html).expect("Unable to write blog index");
}

pub fn copy_folder(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

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
