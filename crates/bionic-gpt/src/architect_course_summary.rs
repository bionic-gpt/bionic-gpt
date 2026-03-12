use ssg_whiz::summaries::*;

pub fn summary() -> Summary {
    Summary {
        source_folder: "architect-course",
        categories: vec![
            Category {
                name: "Zero to Agentic AI Hero".to_string(),
                pages: vec![PageSummary {
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
                    PageSummary {
                        date: "",
                        title: "Choosing a Model",
                        description: "Pick the Granite baseline used throughout the lab.",
                        folder: "architect-course/010-gen-ai-lab/01-choosing-a-model/",
                        markdown: include_str!(
                            "../content/architect-course/010-gen-ai-lab/01-choosing-a-model/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Running an Inference Engine (Ollama)",
                        description: "Connecting to Ollam",
                        folder: "architect-course/010-gen-ai-lab/02-ollama/",
                        markdown: include_str!("../content/architect-course/010-gen-ai-lab/02-ollama/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Running the Bionic Agentic AI Platform",
                        description: "Try it on a Laptop",
                        folder: "architect-course/010-gen-ai-lab/03-docker-compose/",
                        markdown: include_str!(
                            "../content/architect-course/010-gen-ai-lab/03-docker-compose/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Testing Your Model",
                        description: "Run sanity checks to validate prompts and responses.",
                        folder: "architect-course/010-gen-ai-lab/04-testing-model/",
                        markdown: include_str!(
                            "../content/architect-course/010-gen-ai-lab/04-testing-model/index.md"
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
                    PageSummary {
                        date: "",
                        title: "Understanding Tool Calls",
                        description: "When and how to wire structured tool executions.",
                        folder: "architect-course/020-basics-of-tool-calls/010-understanding-tool-calls/",
                        markdown: include_str!(
                            "../content/architect-course/020-basics-of-tool-calls/010-understanding-tool-calls/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Agentic Loop",
                        description: "Why agentic AI is just tool calling in a loop.",
                        folder: "architect-course/020-basics-of-tool-calls/020-agentic-loop/",
                        markdown: include_str!(
                            "../content/architect-course/020-basics-of-tool-calls/020-agentic-loop/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Tool Calls in the UI",
                        description: "Follow tool invocation state transitions inside the console.",
                        folder: "architect-course/020-basics-of-tool-calls/030-tool-calls-in-the-ui/",
                        markdown: include_str!(
                            "../content/architect-course/020-basics-of-tool-calls/030-tool-calls-in-the-ui/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Open Claw Runtime Tools",
                        description: "Runtime capabilities available in Open Claw.",
                        folder: "architect-course/020-basics-of-tool-calls/040-tool-calls-open-claw/",
                        markdown: include_str!(
                            "../content/architect-course/020-basics-of-tool-calls/040-tool-calls-open-claw/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
            Category {
                name: "Common Toolsets".to_string(),
                pages: vec![
                    PageSummary {
                        date: "",
                        title: "Common Toolsets",
                        description: "Runtime capabilities for operating autonomous agents.",
                        folder: "architect-course/025-common-toolsets/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Memory",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/020-memory/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/020-memory/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Documents and Attachments",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/060-documents-and-attachments/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/060-documents-and-attachments/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Sandboxes",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/010-sandboxes/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/010-sandboxes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Scheduled Jobs (Cron)",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/030-scheduled-jobs-cron/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/030-scheduled-jobs-cron/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Skills",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/040-skills/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/040-skills/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "OpenAPI Toolsets",
                        description: "Placeholder page.",
                        folder: "architect-course/025-common-toolsets/050-openapi-toolsets/",
                        markdown: include_str!(
                            "../content/architect-course/025-common-toolsets/050-openapi-toolsets/index.md"
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
                    PageSummary {
                        date: "",
                        title: "Connecting LLMs to External Systems",
                        description: "Conceptual overview of connecting external services.",
                        folder: "architect-course/030-agentic-integrations/030-understanding-integrations/",
                        markdown: include_str!(
                            "../content/architect-course/030-agentic-integrations/030-understanding-integrations/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Connecting an Assistant to Our Integrations",
                        description: "Enable built-in integrations and wire them to an assistant from the console.",
                        folder: "architect-course/030-agentic-integrations/040-connecting-the-integration/",
                        markdown: include_str!(
                            "../content/architect-course/030-agentic-integrations/040-connecting-the-integration/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Using Our Integration",
                        description: "Hands-on walkthrough with the generic integration connector.",
                        folder: "architect-course/030-agentic-integrations/045-using-our-integration/",
                        markdown: include_str!(
                            "../content/architect-course/030-agentic-integrations/045-using-our-integration/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Custom Integrations",
                        description: "Use OpenAPI specs to wire in-house APIs into assistants.",
                        folder: "architect-course/030-agentic-integrations/050-understanding-open-api-specifications/",
                        markdown: include_str!(
                            "../content/architect-course/030-agentic-integrations/050-understanding-open-api-specifications/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Create an Integration",
                        description: "Steps for granting secure Gmail access to assistants.",
                        folder: "architect-course/030-agentic-integrations/055-create-an-integration/",
                        markdown: include_str!(
                            "../content/architect-course/030-agentic-integrations/055-create-an-integration/index.md"
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
                        folder: "architect-course/050-ai-ops/why-kubernetes/",
                        markdown: include_str!(
                            "../content/architect-course/050-ai-ops/why-kubernetes/index.md"
                        ),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                    PageSummary {
                        date: "",
                        title: "Running Kubernetes Lab",
                        description: "Install a local K3s cluster that mirrors production topologies.",
                        folder: "architect-course/050-ai-ops/install-linux/",
                        markdown: include_str!("../content/architect-course/050-ai-ops/install-linux/index.md"),
                        image: None,
                        author_image: None,
                        author: None,
                    },
                ],
            },
        ],
    }
}
