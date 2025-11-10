## Identifying candidate workflows for Agentic Transformation

Taking high stress workflows

![Legacy System](legacy-systems.png "Legacy System")

### Ideally

- Come with an existing API
- Have a good UAT/Testing environment for a pilot
- Have the kind of business benefit that would get a budget anyway

## 1. Get an OPen API machine Readable Specification

OpenAPI specs provide the contract agents rely on.

- Shared artifact for design, engineering, and compliance.
- Generated docs, SDKs, and contract tests fall out automatically.
- Blast radius is clear before anything touches production data.

![Open API](open-api.png "Open API")

## 2. Deploying the Spec

Once the spec exists, publish it where the agent runtime lives (internal gateway, Deploy MCP, or any policy engine).

- Lint the spec and attach metadata (owners, environments, data classification).
- Register the tool so prompts can call it with standard logging and throttling.

## 3. Chat with your legacy system

With the specs and tools published, a chat surface becomes the control plane.

And connecting to an LLM

![Agentic Console](agentic-console.png "Agentic Console")

- Prompts turn into tool invocations with guardrails.
- Agents summarize results and ask clarifying questions when data looks risky.
- Operators accept or reject actions with full visibility into what will run.

## The Chat is the UI

Chat-first interfaces keep work visible and searchable.

- Intent detection happens inline instead of across forms.
- Structured cards show up only when precision is required.
- Conversation history doubles as onboarding and audit trail.

## Authentication and Access

Security controls move with the agent.

- Rotate scoped service accounts or OAuth tokens automatically.
- Enforce RBAC per tool; the orchestrator (Deploy MCP or your own control plane) governs who may invoke each tool.
- Record every action so approvals and dual-control flows stay auditable.
