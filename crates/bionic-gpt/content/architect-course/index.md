# Course Introduction

By the end of this course, you will understand how agentic AI systems combine
models, tools, integrations, sandboxes, skills, memory, retrieval, and human
review into a working enterprise AI platform.

The evals are designed to help teams compare models, prompts, tools, and
deployment patterns before committing to a production design. They can support
decisions about which models are suitable for on-premise, private-cloud, or
approved hosted environments, while also teaching the practical details of
agentic AI that are easy to miss in abstract architecture discussions.

## Enterprise Evals

Enterprise evals use controlled mock services, fixed source data, and known
expected outcomes. That lets you test tool use, reasoning, retrieval,
permissions, and artifact generation without relying on live enterprise systems.

As the course grows, each eval can stand on its own with its own prompt, tools,
data, and success criteria.

## Course outline

- **Set up the workspace:** [run the platform](
  /architect-course/lab/docker-compose/), connect an
  [API provider](/architect-course/lab/api-provider/) or
  [Ollama](/architect-course/lab/ollama/), and
  [test the platform](/architect-course/lab/test-platform/).
- **Understand actions:** learn about [tool calls](
  /architect-course/how-agentic-ai-works/understanding-tool-calls/)
  and [the agentic loop](
  /architect-course/how-agentic-ai-works/agentic-loop/).
- **Create a working environment:** combine [sandboxes](
  /architect-course/ai-computer/sandboxes/), a
  [virtual file system](
  /architect-course/ai-computer/virtual-file-system/), and
  [runtime tools](
  /architect-course/how-agentic-ai-works/runtime-tools/).
- **Make work reusable:** add [skills](
  /architect-course/ai-computer/skills/),
  [memory](/architect-course/ai-computer/memory/), and
  [scheduled tasks](
  /architect-course/ai-computer/scheduled-tasks/).
- **Run enterprise evals:** test [inbox summarization](
  /architect-course/enterprise-evals/inbox-summarization/) and a
  [deep research](
  /architect-course/enterprise-evals/research/).
- **Operate safely:** understand [why Kubernetes](
  /architect-course/deployment-and-operations/why-kubernetes/) and
  [run the Kubernetes lab](
  /architect-course/deployment-and-operations/install-linux/).
