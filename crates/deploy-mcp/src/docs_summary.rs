use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Getting Started".to_string(),
            pages: vec![Page {
                date: "2024-05-07",
                title: "Getting started with Deploy",
                description: "Launch your first AI assistant with Deploy in four steps.",
                folder: "docs/",
                markdown: include_str!("../content/docs/index.md"),
                image: None,
                author: None,
                author_image: None,
            }],
        }],
    }
}
