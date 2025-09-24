use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Launches".to_string(),
            pages: vec![Page {
                date: "2024-05-07",
                title: "Launching Deploy",
                description: "Introducing Deploy, the governance-first AI automation platform.",
                folder: "blog/launching-deploy/",
                markdown: include_str!("../content/blog/launching-deploy.md"),
                image: Some("https://placehold.co/1200x630"),
                author: Some("Alex Rivera"),
                author_image: Some("https://placehold.co/88x88"),
            }],
        }],
    }
}
