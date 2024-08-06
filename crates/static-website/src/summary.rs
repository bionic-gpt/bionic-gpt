use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::blog::{BlogList, BlogListProps, BlogPost, BlogPostProps};
use crate::docs::{Document, DocumentProps};

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

pub fn generate(summary: Summary) {
    let src = Path::new(summary.source_folder);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();

    for category in summary.categories {
        for page in category.pages {
            let html = crate::render_with_props(BlogPost, BlogPostProps { post: page });
            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub fn generate_docs(summary: Summary) {
    let src = Path::new(summary.source_folder);
    let dst = format!("dist/{}", summary.source_folder);
    let dst = Path::new(&dst);
    copy_folder(src, dst).unwrap();

    for category in &summary.categories {
        for page in &category.pages {
            let html = crate::render_with_props(
                Document,
                DocumentProps {
                    summary: summary.clone(),
                    category: category.clone(),
                    doc: *page,
                },
            );
            let file = format!("dist/{}/index.html", page.folder);

            let mut file = File::create(&file).expect("Unable to create file");
            file.write_all(html.as_bytes())
                .expect("Unable to write to file");
        }
    }
}

pub async fn generate_blog_list(summary: Summary) {
    let html = crate::render_with_props(BlogList, BlogListProps { summary });

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
