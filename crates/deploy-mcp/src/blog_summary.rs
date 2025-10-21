use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "Launches".to_string(),
            pages: vec![
                Page {
                    date: "2025-10-21",
                    title: "Vibe Engineering: How does it fit into the SDLC?",
                    description:
                        "Looking at how the MCP protocol works with LLMs from first principals.",
                    folder: "blog/vibe-engineering/",
                    markdown: include_str!("../content/blog/vibe-engineering/index.md"),
                    image: Some("/blog/vibe-engineering/llama-devops.png"),
                    author: Some("Alex Rivera"),
                    author_image: Some("/blog-authors/alex-rivera.png"),
                },
                Page {
                    date: "2025-10-01",
                    title: "Deep Dive: Understanding the MCP protocol using curl",
                    description:
                        "Looking at how the MCP protocol works with LLMs from first principals.",
                    folder: "blog/mcp-explained-with-curl/",
                    markdown: include_str!("../content/blog/mcp-explained-with-curl/index.md"),
                    image: Some("/blog/mcp-explained-with-curl/curl-and-mcp.webp"),
                    author: Some("Alex Rivera"),
                    author_image: Some("/blog-authors/alex-rivera.png"),
                },
            ],
        }],
    }
}
