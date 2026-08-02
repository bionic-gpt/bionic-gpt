# Course Introduction

By the end of this course, you will understand how agentic AI can complete the
following fictional enterprise use case.

## A Fictional Use Case

Imagine you received the following email from your CEO:

```text
From: Maya Chen, Chief Executive Officer
To: AI Strategy Team
Subject: Sovereign AI platform recommendation
Attachment: sovereign-ai-requirements.pdf

Team,

Please evaluate the leading sovereign AI platform vendors against the
attached requirements.

Verify their capabilities using official sources and recommend a
shortlist. I need an executive recommendation, a vendor comparison
spreadsheet, a target architecture, and a steering committee
presentation.

Please also draft my reply to the executive team.

Thanks,
Maya
```

### Sample files

- `ceo-request.eml` — Download sample
- `sovereign-ai-requirements.pdf` — Download sample

You could then ask your AI workspace:

```text
got email from ceo, you do it
```

To complete the request, the AI workspace must:

1. Retrieve the email.
2. Read the attachment.
3. Extract the requirements.
4. Research current vendors.
5. Verify claims against official sources.
6. Analyse deployment, security, integrations, and cost.
7. Generate the recommendation, spreadsheet, architecture, and presentation.
8. Draft a response to the CEO.

## Course outline

- **Set up the workspace:** [run the platform](
  /architect-course/010-gen-ai-lab/03-docker-compose/), connect an
  [API provider](/architect-course/010-gen-ai-lab/035-api-provider/) or
  [Ollama](/architect-course/010-gen-ai-lab/02-ollama/), and
  [test the platform](/architect-course/010-gen-ai-lab/04-testing-model/).
- **Understand actions:** learn about [tool calls](
  /architect-course/020-basics-of-tool-calls/010-understanding-tool-calls/)
  and [the agentic loop](
  /architect-course/020-basics-of-tool-calls/020-agentic-loop/).
- **Create a working environment:** combine [sandboxes](
  /architect-course/025-common-toolsets/010-sandboxes/), a
  [virtual file system](
  /architect-course/025-common-toolsets/060-documents-and-attachments/), and
  [runtime tools](
  /architect-course/020-basics-of-tool-calls/040-tool-calls-open-claw/).
- **Make work reusable:** add [skills](
  /architect-course/025-common-toolsets/040-skills/),
  [memory](/architect-course/025-common-toolsets/020-memory/), and
  [scheduled tasks](
  /architect-course/025-common-toolsets/030-scheduled-jobs-cron/).
- **Connect enterprise systems:** use [integrations](
  /architect-course/030-agentic-integrations/030-understanding-integrations/)
  and [OpenAPI toolsets](
  /architect-course/025-common-toolsets/050-openapi-toolsets/).
- **Keep people in control:** [present results for review](
  /architect-course/027-human-in-the-loop/010-presenting-results/).
- **Operate safely:** understand [why Kubernetes](
  /architect-course/050-ai-ops/why-kubernetes/) and
  [run the Kubernetes lab](
  /architect-course/050-ai-ops/install-linux/).
- **Decide what to build:** evaluate [when a bespoke agent is justified](
  /architect-course/025-common-toolsets/080-when-to-build-an-agent/).
