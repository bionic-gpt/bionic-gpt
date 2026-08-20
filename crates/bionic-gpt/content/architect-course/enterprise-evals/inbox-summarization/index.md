# Inbox Summarization

This eval tests whether the model can inspect an enterprise inbox, identify the
latest relevant request and follow-up, draft a useful response, and avoid
sending it before approval.

The eval provides a controlled email API with fixed messages and expected
behaviour. It is designed to test inbox triage without connecting to a real
mailbox.

[Mockoon](https://mockoon.com/) lets us create a realistic mock REST API without
connecting to a real mailbox. Bionic's eval mocks service runs the OpenAPI spec
as a mock API.

## Test Prompt

Once the integration is available, try:

```text
Review the inbox, identify the latest request that needs a reply, draft a
response asking for the missing security requirements, and do not send it yet.
```

A good result should:

1. List or inspect the available emails.
2. Read the relevant request.
3. Notice the security follow-up.
4. Create a draft response.
5. Avoid calling `sendDraft`.

## Download the Spec

- [Download the OpenAPI spec](/architect-course/enterprise-evals/email-integration.openapi.yaml)

The OpenAPI spec defines the fake email service, provides deterministic example
responses for Mockoon, and is what Bionic uses to turn that service into
callable tools. The default lab already includes the mock API container.

## What the Eval API Provides

The API exposes four operations:

| Operation | Method | Path | Purpose |
| --- | --- | --- | --- |
| `listEmails` | `GET` | `/email/emails` | List recent email messages |
| `getEmail` | `GET` | `/email/emails/{id}` | Read one email |
| `createDraft` | `POST` | `/email/drafts` | Create an email draft |
| `sendDraft` | `POST` | `/email/send` | Queue a draft for sending |

The inbox contains a primary request and a follow-up security review message.
This gives the model enough operational context to test message inspection,
follow-up detection, and draft creation without using a real email provider.

## Add the Integration to Bionic

Download the OpenAPI spec, then go to the admin area in Bionic. Open
**OpenAPI Specs**, add a new spec, and paste or upload the inbox eval YAML.

Return to the app, open **Integrations**, add an integration, and choose the
email or inbox spec you just added.

This eval integration does not require authentication. That keeps the
evaluation focused on whether the model can discover tools, read enterprise
context, and draft a useful response.

That gives us a repeatable enterprise evaluation: the same API, the same data,
and the same expected behaviour every time we test the platform.

## Adding More Mock Integrations

The eval mocks image is intended to grow with the course. Add future mock
systems under their own path prefix and OpenAPI spec under:

```text
infra-as-code/eval-mocks/openapi/
```

If the file should be downloadable from this lesson, also add a copy under the
course assets directory. Keeping the mock routes and OpenAPI specs in the repo
makes the enterprise evaluation repeatable in CI, local development, and shared
demos.
