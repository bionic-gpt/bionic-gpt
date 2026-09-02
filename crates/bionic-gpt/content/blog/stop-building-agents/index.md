Enterprise AI has a scaling problem.

A typical Agentic AI job advert might look something like this.

## Typical Agentic AI Job Advert

```
We're looking for a Senior Agentic AI Engineer with experience in:

- Python, TypeScript and Java
- LangGraph, LlamaIndex and Semantic Kernel
- RAG and vector databases
- PostgreSQL, MongoDB, Redis, Elasticsearch and Neo4j
- Kafka and event-driven architectures
- Docker and Kubernetes
- AWS, Azure or Google Cloud
- LLM evaluation and observability
- Agent memory, planning and tool use
- Multi-agent orchestration
- Guardrails and human-in-the-loop workflows

You will work with business stakeholders to **identify high-value use cases for Agentic AI**.
```

There's something backwards here.

We've specified the solution in extraordinary detail before we've identified the problem.

This creates two problems.

### We build before we know what we need

We've already decided that the answer involves agents, RAG, vector databases, orchestration and infrastructure.

The use case might not need any of it.

### The AI team becomes the bottleneck

Large organisations don't have ten potential AI use cases.

They have thousands.

If every one requires an AI engineer to design, build and deploy an agent, most will never get built.

## The Agentic Platform You May Already Have

A few years ago, giving employees access to an LLM mostly meant giving them a chatbot.

That has changed.

Platforms such as ChatGPT, Claude, Mistral and Bionic are becoming general-purpose agentic workspaces.


![Build versus use existing capabilities](chat-architectures.png "Build versus use existing capabilities")

They increasingly provide:

| Connect | Work | Automate | Platform |
|---|---|---|---|
| Enterprise integrations | Skills | Scheduled tasks | Model selection |
| Web & enterprise search | File & document processing | Memory | Sandboxed execution |
| Authentication & permissions | Code execution | Artifact generation | Security & governance |

A lot of the infrastructure we previously needed to build for an AI application is becoming part of the platform.

So before asking:

> How do we build an agent for this?

Ask:

> Can the platform already do it?

## Many Use Cases Aren't Applications Anymore

Consider:

> Check this supplier document against our security policies and identify any compliance issues.

You could build an application for that.

Or the workspace could read the documents and use a reusable **document comparison skill**.

No new application. No deployment. No engineering backlog.

The same applies to many everyday use cases:

- analyse this spreadsheet
- research these suppliers
- query this database
- investigate this market movement
- prepare a presentation
- process these emails
- produce this monthly report

These are tasks, not necessarily applications.

## Example: Future Bond Rollover

Consider a trader who needs to analyse an upcoming bond futures rollover.

### Integrations

The workspace connects to the relevant positions and market data.

[Screenshot]

### Skills

The trader asks the workspace to analyse the rollover.

Existing skills provide the analytical and reporting capabilities.

[Screenshot]

### Final Result

The workspace performs the analysis and produces the Treasury report.

[Screenshot]

If this needs to happen every month:

> Run this on the first business day of every month and send the report to Treasury.

Now we have a recurring workflow.

We haven't built a new application.

## Fixing the Bottleneck

But the bigger change isn't technical.

It's who solves the next problem.

The first time, our trader might need help from an AI specialist.

They learn how to provide data, use integrations, apply skills, validate the result and automate the workflow.

Next time they have a different problem, they don't necessarily need another AI project.

They can solve it themselves.

Instead of:

`User → Requirements → AI Team → Engineering → Deployment → User`

we increasingly get:

`User → Agentic Workspace → Result`

The central AI team's role changes.

Instead of:

> Tell us your use case and we'll build you an agent.

It becomes:

> We'll give you the platform, integrations, skills and training to solve many of your own problems.

Some problems will still require engineering.

That's fine.

Engineering becomes the escalation path, not the starting point.

**The goal isn't to build thousands of agents.**

**It's to enable thousands of people to solve problems using agents.**