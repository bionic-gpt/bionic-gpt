use crate::generator::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "architect-course",
        categories: vec![
            Category {
                name: "Gen AI Architect Course".to_string(),
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
                        folder: "architect-course/01-gen-ai-lab/01-choosing-a-model/",
                        markdown: include_str!(
                            "../content/architect-course/01-gen-ai-lab/01-choosing-a-model/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Running an Inference Engine (Ollama)",
                        description: "Connecting to Ollam",
                        folder: "architect-course/01-gen-ai-lab/02-ollama/",
                        markdown: include_str!("../content/architect-course/01-gen-ai-lab/02-ollama/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Running the Bionic Agentic AI Platform",
                        description: "Try it on a Laptop",
                        folder: "architect-course/01-gen-ai-lab/03-docker-compose/",
                        markdown: include_str!(
                            "../content/architect-course/01-gen-ai-lab/03-docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Testing Your Model",
                        description: "Run sanity checks to validate prompts and responses.",
                        folder: "architect-course/01-gen-ai-lab/04-testing-model/",
                        markdown: include_str!(
                            "../content/architect-course/01-gen-ai-lab/04-testing-model/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Basics of Tool Calls".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Understanding Tool Calls",
                        description: "When and how to wire structured tool executions.",
                        folder: "architect-course/02-basics-of-tool-calls/010-basics-of-tool-calls/010-understanding-tool-calls/",
                        markdown: include_str!(
                            "../content/architect-course/02-basics-of-tool-calls/010-basics-of-tool-calls/010-understanding-tool-calls/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Tool Calls in the UI",
                        description: "Follow tool invocation state transitions inside the console.",
                        folder: "architect-course/02-basics-of-tool-calls/010-basics-of-tool-calls/020-tool-calls-in-the-ui/",
                        markdown: include_str!(
                            "../content/architect-course/02-basics-of-tool-calls/010-basics-of-tool-calls/020-tool-calls-in-the-ui/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Agentic Integrations".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Connecting LLMs to External Systems",
                        description: "Conceptual overview of connecting external services.",
                        folder: "architect-course/03-agentic-integrations/030-understanding-integrations/",
                        markdown: include_str!(
                            "../content/architect-course/03-agentic-integrations/030-understanding-integrations/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Connecting an Assistant to Our Integrations",
                        description: "Enable built-in integrations and wire them to an assistant from the console.",
                        folder: "architect-course/03-agentic-integrations/040-connecting-the-integration/",
                        markdown: include_str!(
                            "../content/architect-course/03-agentic-integrations/040-connecting-the-integration/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Using Our Integration",
                        description: "Hands-on walkthrough with the generic integration connector.",
                        folder: "architect-course/03-agentic-integrations/045-using-our-integration/",
                        markdown: include_str!(
                            "../content/architect-course/03-agentic-integrations/045-using-our-integration/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Custom Integrations",
                        description: "Use OpenAPI specs to wire in-house APIs into assistants.",
                        folder: "architect-course/03-agentic-integrations/050-understanding-open-api-specifications/",
                        markdown: include_str!(
                            "../content/architect-course/03-agentic-integrations/050-understanding-open-api-specifications/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Create an Integration",
                        description: "Steps for granting secure Gmail access to assistants.",
                        folder: "architect-course/03-agentic-integrations/055-create-an-integration/",
                        markdown: include_str!(
                            "../content/architect-course/03-agentic-integrations/055-create-an-integration/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Agentic RAG (Coming Soon)".to_string(),
                pages: vec![
                    Page {
                        date: "",
                        title: "Agentic RAG Introduction",
                        description: "Core concepts behind our Agentic RAG pattern.",
                        folder: "architect-course/04-agentic-rag/060-understanding-rag/",
                        markdown: include_str!(
                            "../content/architect-course/04-agentic-rag/060-understanding-rag/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Loading Agentic RAG Datasets",
                        description: "Ingestion patterns and guardrails for Agentic RAG datasets.",
                        folder: "architect-course/04-agentic-rag/070-loading-rag-datasets/",
                        markdown: include_str!(
                            "../content/architect-course/04-agentic-rag/070-loading-rag-datasets/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Testing an Agentic RAG Pipeline in the UI (Use Case)",
                        description: "Guided validation scenario for Agentic RAG pipelines in the console.",
                        folder: "architect-course/04-agentic-rag/080-testing-a-rag-pipeline-in-the-ui/",
                        markdown: include_str!(
                            "../content/architect-course/04-agentic-rag/080-testing-a-rag-pipeline-in-the-ui/index.md"
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
                        title: "Why Kubernetes",
                        description: "Why the platform relies on Kubernetes instead of standalone Docker setups.",
                        folder: "architect-course/05-ai-ops/why-kubernetes/",
                        markdown: include_str!(
                            "../content/architect-course/05-ai-ops/why-kubernetes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    Page {
                        date: "",
                        title: "Running Kubernetes Lab",
                        description: "Install a local K3s cluster that mirrors production topologies.",
                        folder: "architect-course/05-ai-ops/install-linux/",
                        markdown: include_str!("../content/architect-course/05-ai-ops/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
