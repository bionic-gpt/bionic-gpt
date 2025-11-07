## Wouldn't it be nice if...

Legacy workflows often force people through fragile terminal screens or bespoke web forms. Making those systems agentic means adding a thin layer that:

- Translates natural-language requests into authenticated API calls.
- Validates inputs before payloads reach the core system.
- Logs every decision for operations, audit, and rollback.

Platforms such as Deploy MCP can host that layer, but the real unlock is the methodology: capture the business intent, expose the system via specs, and let the agent call it safely.

> Book wednesday off next week

![Legacy System](legacy-system.png "Legacy System")

> Do a bond roll over

![Legacy System](trading-system.png "Legacy System")

## What Agentic AI Looks Like

Agentic patterns replace hand-built scripts with prompt-to-plan execution.

- Plans multi-step workflows and branches on schema-validated responses.
- Calls internal or external APIs through a single set of auth controls.
- Emits structured traces for replay, debugging, and compliance.

Once the tooling is proven, curated prompts let operators unwind trades, provision infrastructure, or open support tickets without waiting on engineering, while engineers keep ownership of the tools and policies the agent can invoke.

![Codex](codex.png "Codex")

## First Spec out the Systems

OpenAPI specs provide the contract agents rely on.

- Shared artifact for design, engineering, and compliance.
- Generated docs, SDKs, and contract tests fall out automatically.
- Blast radius is clear before anything touches production data.

![Open API](open-api.png "Open API")

## Generating a Spec with Chat GPT

> Use GPT to draft the boring bits, then review it like any pull request.

When a system only exposes a UI, we can still infer the spec.

1. Capture a user session (DOM plus network traces).
2. Describe the workflow, roles, and failure cases.
3. Ask GPT to draft the OpenAPI snippet.
4. Tighten naming, auth, and validation rules before publishing.

```json
{
  "openapi": "3.1.0",
  "info": {
    "title": "Bond Desk",
    "version": "0.1.0"
  },
  "paths": {
    "/rollover": {
      "post": {
        "summary": "Initiate bond rollover",
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "cusip": { "type": "string" },
                  "quantity": { "type": "integer" }
                },
                "required": ["cusip", "quantity"]
              }
            }
          }
        }
      }
    }
  }
}
```

## Deploying the Spec

Once the spec exists, publish it where the agent runtime lives (internal gateway, Deploy MCP, or any policy engine).

- Lint the spec and attach metadata (owners, environments, data classification).
- Register the tool so prompts can call it with standard logging and throttling.

## Bringing in Agnetic RAG

Legacy systems often hide context in PDFs, tickets, or runbooks. Agentic Retrieval-Augmented Generation (RAG) keeps that knowledge next to the tools.

- Index procedures, exception codes, and policy docs.
- Ground the agent’s reasoning before it calls a tool.
- Return citations so operators can verify the source quickly.

## Chat with your legacy system

With the specs and tools published, a chat surface becomes the control plane.

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
