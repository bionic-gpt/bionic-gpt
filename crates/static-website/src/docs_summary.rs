use crate::summary::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Introduction".to_string(),
            pages: vec![Page {
                date: "",
                title: "",
                description: "",
                folder: "docs/community-edition/introduction/",
                markdown: include_str!("../docs/community-edition/introduction/index.md"),
            }],
        }],
    }
}
