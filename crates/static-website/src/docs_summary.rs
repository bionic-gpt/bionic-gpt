use crate::summary::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![Category {
            name: "Introduction".to_string(),
            pages: vec![
                Page {
                    date: "",
                    title: "Intro",
                    description: "",
                    folder: "docs/",
                    markdown: include_str!("../docs/index.md"),
                    image: None,
                },
                Page {
                    date: "",
                    title: "Docker Compose",
                    description: "",
                    folder: "docs/community-edition/docker-compose/",
                    markdown: include_str!("../docs/community-edition/docker-compose/index.md"),
                    image: None,
                },
            ],
        }],
    }
}
