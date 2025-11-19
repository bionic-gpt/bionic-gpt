use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "architect-course",
        categories: vec![
            Category {
                name: "Architect Course".to_string(),
                pages: vec![Page {
                    date: "",
                    title: "Course Overview",
                    description: "Introduction to the architect curriculum.",
                    folder: "architect-course/",
                    markdown: include_str!("../content/architect-course/index.md"),
                    image: None,
                    author_image: None,
                    author: None,
                }],
            },
            Category {
                name: "Setting up a Lab".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Choosing a Model",
                        description: "Pick the Granite baseline used throughout the lab.",
                        folder: "architect-course/01-gen-ai-lab/choosing-a-model/",
                        markdown: include_str!(
                            "../content/architect-course/01-gen-ai-lab/choosing-a-model/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Running an Inference Engine (Ollama)",
                        description: "Connecting to Ollam",
                        folder: "architect-course/01-gen-ai-lab/ollama/",
                        markdown: include_str!("../content/architect-course/01-gen-ai-lab/ollama/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Attaching the UI",
                        description: "Try it on a Laptop",
                        folder: "architect-course/01-gen-ai-lab/docker-compose/",
                        markdown: include_str!(
                            "../content/architect-course/01-gen-ai-lab/docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Agentic Ai Experiments".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Understanding Tool Calls",
                        description: "When and how to wire structured tool executions.",
                        folder: "architect-course/02-agentic-ai/010-understanding-tool-calls/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/010-understanding-tool-calls/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Tool Calls in the UI",
                        description: "Follow tool invocation state transitions inside the console.",
                        folder: "architect-course/02-agentic-ai/020-tool-calls-in-the-ui/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/020-tool-calls-in-the-ui/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Assistants (Prompt Engineering)",
                        description: "Layer prompts and guardrails for reliable assistants.",
                        folder: "architect-course/02-agentic-ai/030-assistants-prompt-engineering/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/030-assistants-prompt-engineering/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Credit CXard Categorisation Assistant",
                        description: "Example workflow for classifying credit card support cases.",
                        folder: "architect-course/02-agentic-ai/040-credit-cxard-categorisation-assistant/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/040-credit-cxard-categorisation-assistant/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Understanding RAG",
                        description: "Core concepts behind retrieval augmented generation.",
                        folder: "architect-course/02-agentic-ai/050-understanding-rag/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/050-understanding-rag/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Loading RAG Datasets",
                        description: "Ingestion patterns and guardrails for datasets.",
                        folder: "architect-course/02-agentic-ai/060-loading-rag-datasets/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/060-loading-rag-datasets/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Testing a RAG Pipeline in the UI (Use Case)",
                        description: "Guided validation scenario for pipelines in the console.",
                        folder: "architect-course/02-agentic-ai/070-testing-a-rag-pipeline-in-the-ui/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/070-testing-a-rag-pipeline-in-the-ui/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Understanding Integrations",
                        description: "Conceptual overview of connecting external services.",
                        folder: "architect-course/02-agentic-ai/080-understanding-integrations/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/080-understanding-integrations/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Understanding Open API Specifications",
                        description: "Write and maintain high quality OpenAPI contracts.",
                        folder: "architect-course/02-agentic-ai/090-understanding-open-api-specifications/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/090-understanding-open-api-specifications/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting to Gmail",
                        description: "Steps for granting secure Gmail access to assistants.",
                        folder: "architect-course/02-agentic-ai/100-connecting-to-gmail/",
                        markdown: include_str!(
                            "../content/architect-course/02-agentic-ai/100-connecting-to-gmail/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Deployment and Operations".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Quick Install (Linux)",
                        description: "Quick Install (Linux)",
                        folder: "architect-course/03-ai-ops/install-linux/",
                        markdown: include_str!("../content/architect-course/03-ai-ops/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
