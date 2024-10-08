use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2024-10-09",
                    title: "Create a Data Lakehouse with Trino, Iceberg, S3, Parquet and Kubernetes",
                    description: "A Data Lakehouse with Trino, Iceberg, S3, Parquet and Kubernetes",
                    folder: "blog/ai-lakehouse/",
                    markdown: include_str!("../content/blog/ai-lakehouse/index.md"),
                    image: Some("/blog/ai-agents/ai-agents.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-09-26",
                    title: "AI Agents: Transforming Business Operations",
                    description: "A detailed guide to AI agents, what they are, how they can be used and how they will shape the future",
                    folder: "blog/ai-agents/",
                    markdown: include_str!("../content/blog/ai-agents/index.md"),
                    image: Some("/blog/ai-agents/ai-agents.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-09-20",
                    title: "What Are Guardrails",
                    description: "A guide to AI guardrails, what they are, what they do and the types of guardrail you can implement",
                    folder: "blog/guardrails/",
                    markdown: include_str!("../content/blog/guardrails/index.md"),
                    image: Some("/blog/guardrails/guardrails.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-08-21",
                    title: "How Enterprises Can Prepare for Generative AI",
                    description: "A comprehensive guide for businesses to effectively integrate and leverage Generative AI. Emphasising the importance of strategic planning, stakeholder engagement, and infrastructure readiness. Outlining the potential benefits, while cautioning about risks like algorithmic bias and data privacy concerns. Highlighting the need for robust governance, multidisciplinary teams, and continuous monitoring to ensure AI initiatives align with business goals and deliver measurable value.",
                    folder: "blog/preparing-for-gen-ai/",
                    markdown: include_str!("../content/blog/preparing-for-gen-ai/index.md"),
                    image: Some("/blog/preparing-for-gen-ai/enterprise.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-16",
                    title: "Create A ChatBot On Your Data",
                    description: "Create A ChatBot On Your Data",
                    folder: "blog/api-chatbot/",
                    markdown: include_str!("../content/blog/api-chatbot/index.md"),
                    image: Some("/blog/api-chatbot/chatbot-screenshot.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-12",
                    title: "The Perfect RAG Use Case?",
                    description: "The Perfect RAG Use Case?",
                    folder: "blog/code-rag-usecase/",
                    markdown: include_str!("../content/blog/code-rag-usecase/index.md"),
                    image: Some("/blog/code-rag-usecase/codeimage.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-11",
                    title: "Building SaaS applications for highly regulated industries using Confidential Computing",
                    description: "Building SaaS applications for highly regulated industries using Confidential Computing",
                    folder: "blog/confidential-saas/",
                    markdown: include_str!("../content/blog/confidential-saas/index.md"),
                    image: Some("/blog/confidential-saas/kubernetes.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-05-28",
                    title: "Why companies are banning Chat-GPT",
                    description: "",
                    folder: "blog/banning-chat-gpt/",
                    markdown: include_str!("../content/blog/banning-chat-gpt/index.md"),
                    image: Some("/blog/banning-chat-gpt/chat-gpt-banned.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-04-10",
                    title: "Model Denial of Service prevention for  production LLM applications",
                    description: "",
                    folder: "blog/model-denial-of-service/",
                    markdown: include_str!("../content/blog/model-denial-of-service/index.md"),
                    image: Some("/blog/model-denial-of-service/model-denial-of-service.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-12-04",
                    title: "bionicGPT Integration with Jupyter",
                    description: "",
                    folder: "blog/jupyter/",
                    markdown: include_str!("../content/blog/jupyter/index.md"),
                    image: Some("/blog/jupyter/llama-jupyter.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2023-10-30",
                    title: "Hardware requirements for LLM's in production",
                    description: "What hardware is required to put LLM's into production?",
                    folder: "blog/llm-hardware/",
                    markdown: include_str!("../content/blog/llm-hardware/index.md"),
                    image: Some("/blog/llm-hardware/multi-gpu-llm-setup.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-10-13",
                    title: "Why you don't need a specialist Vector Database",
                    description: "Questioning the use of adding another database to your system",
                    folder: "blog/you-dont-need-a-vector-database/",
                    markdown: include_str!("../content/blog/you-dont-need-a-vector-database/index.md"),
                    image: Some("/blog/you-dont-need-a-vector-database/postgres-vs-vector.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-10-01",
                    title: "Understanding Quantisation in Large Language Models (LLMs)",
                    description: "Understanding Quantisation in Large Language Models (LLMs)",
                    folder: "blog/quantisation/",
                    markdown: include_str!("../content/blog/quantisation/index.md"),
                    image: Some("/blog/quantisation/futuristic-llama.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2023-09-21",
                    title: "What is Retrieval Augmented Generation?",
                    description: "A more low level guide to Retrieval Augmented Generation",
                    folder: "blog/retrieval-augmented-generation/",
                    markdown: include_str!("../content/blog/retrieval-augmented-generation/index.md"),
                    image: Some("/blog/retrieval-augmented-generation/rag-llama.webp"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
            ],
        }],
    }
}
