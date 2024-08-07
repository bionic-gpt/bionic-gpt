use crate::summary::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "content/blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2024-07-16",
                    title: "Create A ChatBot On Your Data",
                    description: "Create A ChatBot On Your Data",
                    folder: "privacy",
                    markdown: include_str!("../content/pages/privacy.md"),
                    image: None,
                    author_image: None,
                    author: None,
                },
                Page {
                    date: "2024-07-12",
                    title: "The Perfect RAG Use Case?",
                    description: "The Perfect RAG Use Case?",
                    folder: "terms",
                    markdown: include_str!("../content/pages/terms.md"),
                    image: None,
                    author_image: None,
                    author: None,
                },
            ],
        }],
    }
}
