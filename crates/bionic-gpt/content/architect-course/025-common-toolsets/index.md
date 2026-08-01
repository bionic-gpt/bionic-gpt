# From Chatbot to AI Computer

Modern AI chat systems are becoming working environments, not just text interfaces wrapped around a model.

An **AI computer** is a conversational workspace in which a model can inspect and create files, execute commands or code, use installed tools and skills, maintain working context, connect to external systems, schedule future work, and produce finished artifacts.

Here, “AI computer” describes the software environment around the model. It does not mean AI-specific hardware, a GPU workstation, or a consumer “AI PC.”

## The Parts of an AI Computer

| Component | Role |
| --- | --- |
| Chat interface | User interface |
| Model | Reasoning and control |
| Sandbox/runtime | Compute environment |
| Files | Working storage |
| Skills and tools | Software |
| Integrations | Network and enterprise access |
| Memory/projects | Persistent context |
| Scheduled tasks | Scheduler |
| Artifacts | Outputs |

The model reasons about a request, but the surrounding environment gives that reasoning somewhere to act. It can inspect uploaded files, create intermediate results, run tools, preserve useful context, and return a document, spreadsheet, image, or other artifact.

## From Conversation to Work

Once chat includes a working directory, execution, tools, generated outputs, and project context, many workflows no longer need a separately coded agent application. The conversational environment can coordinate the same multi-step work:

1. Understand the request and inspect the available context.
2. Read files or retrieve information from a connected system.
3. Use tools, skills, or code to transform that information.
4. Check intermediate results and correct mistakes.
5. Save and return a finished artifact.

This does not remove the need for software engineering. The runtime still needs clear permissions, isolation, reliable tools, and controls around external access. It means that the reusable working environment can provide much of the orchestration that previously had to be built into each agent.

## The Sandbox Is One Component

A sandbox gives the model an isolated place to execute commands and manipulate files. It is the compute environment inside the AI computer, not the whole computer.

The complete working environment also includes files, skills, memory, integrations, scheduling, and artifacts. The following lessons examine those parts and the boundaries between them.
