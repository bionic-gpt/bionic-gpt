use crate::summary::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "",
                    title: "Create A ChatBot On Your Data",
                    description: "",
                    folder: "blog/api-chatbot/",
                    markdown: include_str!("../blog/api-chatbot/index.md"),
                    image: Some("chatbot-screenshot.png"),
                },
                Page {
                    date: "",
                    title: "The Perfect RAG Use Case?",
                    description: "",
                    folder: "blog/code-rag-usecase/",
                    markdown: include_str!("../blog/code-rag-usecase/index.md"),
                    image: Some("codeimage.png"),
                },
                Page {
                    date: "",
                    title: "Building SaaS applications for highly regulated industries using Confidential Computing",
                    description: "",
                    folder: "blog/confidential-saas/",
                    markdown: include_str!("../blog/confidential-saas/index.md"),
                    image: Some("kubernetes.png"),
                },
                Page {
                    date: "",
                    title: "Why companies are banning Chat-GPT",
                    description: "",
                    folder: "blog/banning-chat-gpt/",
                    markdown: include_str!("../blog/banning-chat-gpt/index.md"),
                    image: Some("chat-gpt-banned.png"),
                },
                Page {
                    date: "",
                    title: "Model Denial of Service prevention for  production LLM applications",
                    description: "",
                    folder: "blog/model-denial-of-service/",
                    markdown: include_str!("../blog/model-denial-of-service/index.md"),
                    image: Some("model-denial-of-service.png"),
                },
                Page {
                    date: "",
                    title: "bionicGPT Integration with Jupyter",
                    description: "",
                    folder: "blog/jupyter/",
                    markdown: include_str!("../blog/jupyter/index.md"),
                    image: Some("llama-jupyter.jpg"),
                },
                Page {
                    date: "",
                    title: "Hardware requirements for LLM's in production",
                    description: "What hardware is required to put LLM's into production?",
                    folder: "blog/llm-hardware/",
                    markdown: include_str!("../blog/llm-hardware/index.md"),
                    image: Some("multi-gpu-llm-setup.jpg"),
                },
                Page {
                    date: "",
                    title: "Why you don't need a specialist Vector Database",
                    description: "Questioning the use of adding another database to your system",
                    folder: "blog/you-dont-need-a-vector-database/",
                    markdown: include_str!("../blog/you-dont-need-a-vector-database/index.md"),
                    image: Some("postgres-vs-vector.jpg"),
                },
                Page {
                    date: "",
                    title: "Understanding Quantisation in Large Language Models (LLMs)",
                    description: "Understanding Quantisation in Large Language Models (LLMs)",
                    folder: "blog/quantisation/",
                    markdown: include_str!("../blog/quantisation/index.md"),
                    image: Some("futuristic-llama.jpg"),
                },
                Page {
                    date: "",
                    title: "What is Retrieval Augmented Generation?",
                    description: "A more low level guide to Retrieval Augmented Generation",
                    folder: "blog/retrieval-augmented-generation/",
                    markdown: include_str!("../blog/retrieval-augmented-generation/index.md"),
                    image: Some("rag-llama.webp"),
                },
            ],
        }],
    }
}
