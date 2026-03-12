use ssg_whiz::summaries::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "pages",
        categories: vec![Category {
            name: "Company".to_string(),
            pages: vec![
                PageSummary {
                    date: "2024-05-07",
                    title: "Privacy Policy",
                    description: "Deploy privacy commitments",
                    folder: "privacy",
                    markdown: include_str!("../content/pages/privacy.md"),
                    image: None,
                    author_image: None,
                    author: None,
                },
                PageSummary {
                    date: "2024-05-07",
                    title: "Terms of Service",
                    description: "Deploy terms of service",
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
