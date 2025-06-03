use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "blog",
        categories: vec![Category {
            name: "TOFU".to_string(),
            pages: vec![
                Page {
                    date: "2025-02-08",
                    title: "The Future Opportunities for Junior Developers in the Age of AI Coding",
                    description: "Explore how AI coding tools create new opportunities for junior developers, from an architect's perspective on career growth.",
                    folder: "blog/junior-developers/",
                    markdown: include_str!("../content/blog/junior-developers/index.md"),
                    image: Some("/blog/junior-developers/junior-developer.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-10-10",
                    title: "Real-Time Content Alerting: A Smarter Approach to Monitoring Harmful Content in Large Language Models",
                    description: "Learn how to implement real-time content monitoring and alerting systems for harmful content in large language models.",
                    folder: "blog/streaming/",
                    markdown: include_str!("../content/blog/streaming/index.md"),
                    image: Some("/blog/streaming/contentMonitoring.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page  {
                    date: "2024-10-10",
                    title: "The Road to Autonomy: How AI in Software Development Mirrors Autonomous Driving Levels",
                    description: "Discover how AI in software development follows autonomous driving levels, from basic assistance to full automation.",
                    folder: "blog/ai-coding-automation/",
                    markdown: include_str!("../content/blog/ai-coding-automation/index.md"),
                    image: Some("/blog/ai-coding-automation/llama-coder.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-10-09",
                    title: "Create a Data Lakehouse with Trino, Iceberg, S3, Parquet and Kubernetes",
                    description: "Build a modern data lakehouse architecture using Trino, Iceberg, S3, Parquet and Kubernetes for scalable analytics.",
                    folder: "blog/ai-lakehouse/",
                    markdown: include_str!("../content/blog/ai-lakehouse/index.md"),
                    image: Some("/blog/ai-lakehouse/lakehouse-architecture.png"),
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
                    description: "Comprehensive guide to AI guardrails: what they are, how they work, and the different types you can implement for safety.",
                    folder: "blog/guardrails/",
                    markdown: include_str!("../content/blog/guardrails/index.md"),
                    image: Some("/blog/guardrails/guardrails.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-08-21",
                    title: "How Enterprises Can Prepare for Generative AI",
                    description: "Strategic guide for enterprises to integrate Generative AI effectively, covering planning, governance, and risk management.",
                    folder: "blog/preparing-for-gen-ai/",
                    markdown: include_str!("../content/blog/preparing-for-gen-ai/index.md"),
                    image: Some("/blog/preparing-for-gen-ai/enterprise.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-16",
                    title: "Create A ChatBot On Your Data",
                    description: "Step-by-step tutorial to build a custom chatbot using your own data with practical examples and implementation tips.",
                    folder: "blog/api-chatbot/",
                    markdown: include_str!("../content/blog/api-chatbot/index.md"),
                    image: Some("/blog/api-chatbot/chatbot-screenshot.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-12",
                    title: "The Perfect RAG Use Case?",
                    description: "Explore the ideal use case for Retrieval Augmented Generation (RAG) and why code-based applications might be perfect.",
                    folder: "blog/code-rag-usecase/",
                    markdown: include_str!("../content/blog/code-rag-usecase/index.md"),
                    image: Some("/blog/code-rag-usecase/codeimage.png"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2024-07-11",
                    title: "Building SaaS applications for highly regulated industries using Confidential Computing",
                    description: "Learn how to build secure SaaS applications for regulated industries using confidential computing and privacy-preserving tech.",
                    folder: "blog/confidential-saas/",
                    markdown: include_str!("../content/blog/confidential-saas/index.md"),
                    image: Some("/blog/confidential-saas/kubernetes.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-05-28",
                    title: "Why companies are banning Chat-GPT",
                    description: "Understand why companies are banning ChatGPT and the security, privacy, and compliance concerns driving these decisions.",
                    folder: "blog/banning-chat-gpt/",
                    markdown: include_str!("../content/blog/banning-chat-gpt/index.md"),
                    image: Some("/blog/banning-chat-gpt/chat-gpt-banned.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2024-04-10",
                    title: "Model Denial of Service prevention for  production LLM applications",
                    description: "Protect your production LLM applications from model denial of service attacks with effective prevention strategies.",
                    folder: "blog/model-denial-of-service/",
                    markdown: include_str!("../content/blog/model-denial-of-service/index.md"),
                    image: Some("/blog/model-denial-of-service/model-denial-of-service.png"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-12-04",
                    title: "bionicGPT Integration with Jupyter",
                    description: "Integrate bionicGPT with Jupyter notebooks for enhanced data science workflows and AI-powered analysis capabilities.",
                    folder: "blog/jupyter/",
                    markdown: include_str!("../content/blog/jupyter/index.md"),
                    image: Some("/blog/jupyter/llama-jupyter.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2023-10-30",
                    title: "Hardware requirements for LLM's in production",
                    description: "Complete guide to hardware requirements for deploying large language models in production environments effectively.",
                    folder: "blog/llm-hardware/",
                    markdown: include_str!("../content/blog/llm-hardware/index.md"),
                    image: Some("/blog/llm-hardware/multi-gpu-llm-setup.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-10-13",
                    title: "Why you don't need a specialist Vector Database",
                    description: "Why PostgreSQL with vector extensions might be better than specialized vector databases for most AI applications.",
                    folder: "blog/you-dont-need-a-vector-database/",
                    markdown: include_str!("../content/blog/you-dont-need-a-vector-database/index.md"),
                    image: Some("/blog/you-dont-need-a-vector-database/postgres-vs-vector.jpg"),
                    author_image: Some("/blog-authors/ian-purton.jpeg"),
                    author: Some("Ian Purton")
                },
                Page {
                    date: "2023-10-01",
                    title: "Understanding Quantisation in Large Language Models (LLMs)",
                    description: "Deep dive into quantization techniques for large language models, reducing memory usage while maintaining performance.",
                    folder: "blog/quantisation/",
                    markdown: include_str!("../content/blog/quantisation/index.md"),
                    image: Some("/blog/quantisation/futuristic-llama.jpg"),
                    author_image: Some("/blog-authors/dio.jpeg"),
                    author: Some("Kulbinder Dio")
                },
                Page {
                    date: "2023-09-21",
                    title: "What is Retrieval Augmented Generation?",
                    description: "Technical deep-dive into Retrieval Augmented Generation (RAG) architecture, implementation details, and best practices.",
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
