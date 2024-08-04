use crate::summary::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![Page {
                date: "",
                title: "Why companies are banning Chat-GPT",
                description: "",
                folder: "blog/banning-chat-gpt/",
                markdown: include_str!("../blog/banning-chat-gpt/index.md"),
                image: Some("chat-gpt-banned.png"),
            }],
        }],
    }
}
