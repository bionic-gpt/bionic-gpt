use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "content/blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2024-07-16",
                    title: "Privacy Policy",
                    description: "Privacy Policy",
                    folder: "privacy",
                    markdown: include_str!("../content/pages/privacy.md"),
                    image: None,
                    author_image: None,
                    author: None,
                },
                Page {
                    date: "2024-07-12",
                    title: "Terms and Conditions",
                    description: "Terms and Conditions",
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
