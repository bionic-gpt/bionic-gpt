use std::fs;

use crate::routes::docs::Index;
use axum::response::Html;
use axum::Router;
use axum_extra::routing::RouterExt;

pub fn routes() -> Router {
    Router::new().typed_get(index)
}

pub async fn index(Index { section, title }: Index) -> Html<String> {
    let sections = parse_directory("docs");
    for section in sections {
        println!("Section: {}", section.section);
        for title in section.titles {
            println!("  Title: {}", title);
        }
    }

    Html(format!("{} {}", section, title))
}

struct Section {
    section: String,
    titles: Vec<String>,
}

fn parse_directory(dir_path: &str) -> Vec<Section> {
    let mut sections = Vec::new();

    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let section = path.file_name().unwrap().to_str().unwrap().to_string();
            let mut titles = Vec::new();
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap();
                let file_path = file.path();
                if file_path.is_file() && file_path.extension().unwrap() == "md" {
                    let title = file_path.file_name().unwrap().to_str().unwrap().to_string();
                    titles.push(title);
                }
            }
            sections.push(Section { section, titles });
        }
    }

    sections
}
