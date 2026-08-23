use ssg_whiz::summaries::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "architect-course",
        categories: vec![
            Category {
                name: "Agentic AI Architecture Course".to_string(),
                pages: vec![PageSummary {
                    date: "",
                    title: "Course Introduction",
                    description:
                        "A practical course for AI leads and enterprise architects. Learn how models, tools, integrations, sandboxes, skills, memory and governance combine into a modern agentic AI platform.",
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
                    PageSummary {
                        date: "",
                        title: "Running the Bionic Agentic AI Platform",
                        description: "Try it on a Laptop",
                        folder: "architect-course/lab/docker-compose/",
                        markdown: include_str!(
                            "../content/architect-course/lab/docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Option 1: Use an API Provider",
                        description: "Connect Bionic to a hosted model provider.",
                        folder: "architect-course/lab/api-provider/",
                        markdown: include_str!(
                            "../content/architect-course/lab/api-provider/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Option 2: Run Locally with Ollama",
                        description: "Run and configure a local model with Ollama.",
                        folder: "architect-course/lab/ollama/",
                        markdown: include_str!(
                            "../content/architect-course/lab/ollama/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Test the Platform",
                        description: "Validate tools, files, and artifacts in Bionic.",
                        folder: "architect-course/lab/test-platform/",
                        markdown: include_str!(
                            "../content/architect-course/lab/test-platform/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "How Agentic AI Works".to_string(),
                pages: vec![
                    PageSummary {
                        date: "",
                        title: "Understanding Tool Calls",
                        description: "When and how to wire structured tool executions.",
                        folder: "architect-course/how-agentic-ai-works/understanding-tool-calls/",
                        markdown: include_str!(
                            "../content/architect-course/how-agentic-ai-works/understanding-tool-calls/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "The Agentic Loop",
                        description: "Why agentic AI is just tool calling in a loop.",
                        folder: "architect-course/how-agentic-ai-works/agentic-loop/",
                        markdown: include_str!(
                            "../content/architect-course/how-agentic-ai-works/agentic-loop/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "The AI Computer".to_string(),
                pages: vec![
                    PageSummary {
                        date: "",
                        title: "From Chatbot to AI Computer",
                        description:
                            "How conversational systems become complete working environments.",
                        folder: "architect-course/ai-computer/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Sandboxes",
                        description:
                            "Isolated filesystems and command execution for model-driven work.",
                        folder: "architect-course/ai-computer/sandboxes/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/sandboxes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Virtual File System",
                        description:
                            "Shared working storage for uploads, datasets, skills, and generated artifacts.",
                        folder: "architect-course/ai-computer/virtual-file-system/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/virtual-file-system/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Runtime Tools",
                        description:
                            "How code execution and tool discovery expand an AI computer.",
                        folder: "architect-course/how-agentic-ai-works/runtime-tools/",
                        markdown: include_str!(
                            "../content/architect-course/how-agentic-ai-works/runtime-tools/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Datasets",
                        description:
                            "Reusable, searchable knowledge collections for grounded model responses.",
                        folder: "architect-course/ai-computer/datasets/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/datasets/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Skills",
                        description: "Packaged instructions for repeatable workflows.",
                        folder: "architect-course/ai-computer/skills/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/skills/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Memory and Context",
                        description: "Persistent context across turns and sessions.",
                        folder: "architect-course/ai-computer/memory/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/memory/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Scheduled Tasks",
                        description: "Future and recurring work initiated by the runtime.",
                        folder: "architect-course/ai-computer/scheduled-tasks/",
                        markdown: include_str!(
                            "../content/architect-course/ai-computer/scheduled-tasks/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Enterprise Evals".to_string(),
                pages: vec![
                    PageSummary {
                        date: "",
                        title: "Inbox Summarization",
                        description:
                            "Evaluate whether the model can inspect an inbox, identify the latest request and follow-up, and draft the right response.",
                        folder:
                            "architect-course/enterprise-evals/inbox-summarization/",
                        markdown: include_str!(
                            "../content/architect-course/enterprise-evals/inbox-summarization/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Deep Research",
                        description:
                            "Evaluate whether the model can use a search integration, inspect source results, and produce an executive briefing.",
                        folder: "architect-course/enterprise-evals/research/",
                        markdown: include_str!(
                            "../content/architect-course/enterprise-evals/research/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Enterprise Database Access",
                        description:
                            "Evaluate whether the model can inspect a live database, discover schemas, run readonly SQL, and produce a grounded operational report.",
                        folder: "architect-course/enterprise-evals/database/",
                        markdown: include_str!(
                            "../content/architect-course/enterprise-evals/database/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Dashboard Generation",
                        description:
                            "Evaluate whether the model can analyse structured data and produce a focused dashboard artifact.",
                        folder: "architect-course/enterprise-evals/dashboard-builder/",
                        markdown: include_str!(concat!(
                            env!("OUT_DIR"),
                            "/dashboard-builder-page.md"
                        )),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Document Comparison",
                        description:
                            "Evaluate a multi-format vendor package against a procurement rubric.",
                        folder: "architect-course/enterprise-evals/document-validation/",
                        markdown: include_str!(
                            "../content/architect-course/enterprise-evals/document-validation/index.md"
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
                    PageSummary {
                        date: "",
                        title: "Why Kubernetes",
                        description: "Why the platform relies on Kubernetes instead of standalone Docker setups.",
                        folder: "architect-course/deployment-and-operations/why-kubernetes/",
                        markdown: include_str!(
                            "../content/architect-course/deployment-and-operations/why-kubernetes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Running the Kubernetes Lab",
                        description: "Install a local K3s cluster that mirrors production topologies.",
                        folder: "architect-course/deployment-and-operations/install-linux/",
                        markdown: include_str!("../content/architect-course/deployment-and-operations/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
