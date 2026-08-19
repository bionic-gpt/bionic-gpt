# Simulated Email Integration

To evaluate an enterprise AI platform repeatably, we need simulated enterprise
systems. The use case starts with an email from the CEO, so the first simulated
integration is an email API.

[Mockoon](https://mockoon.com/) lets us create a realistic mock REST API without
connecting to a real mailbox. Bionic's Docker Compose lab includes a dedicated
`eval-mocks` container that runs the OpenAPI spec as a mock API.

## Download the Spec

- [Download the OpenAPI spec](/architect-course/testing-our-use-case/email-integration.openapi.yaml)

The OpenAPI spec defines the fake email service, provides deterministic example
responses for Mockoon, and is what Bionic uses to turn that service into
callable tools. The default lab already includes the mock API container.

## What the Simulated API Provides

The API exposes four operations:

| Operation | Method | Path | Purpose |
| --- | --- | --- | --- |
| `listEmails` | `GET` | `/email/emails` | List recent email messages |
| `getEmail` | `GET` | `/email/emails/{id}` | Read one email |
| `createDraft` | `POST` | `/email/drafts` | Create an email draft |
| `sendDraft` | `POST` | `/email/send` | Queue a draft for sending |

The inbox contains a CEO request and a follow-up security review message. This
gives the model enough operational context to test an RFX-style workflow without
using a real email provider.

## Run it with Docker Compose

The course Docker Compose file includes the Bionic eval mocks image:

```yaml
eval-mocks:
  image: ghcr.io/bionic-gpt/bionicgpt-eval-mocks:1.12.15
  ports:
    - "3100:3100"
```

Start the lab as usual:

```bash
docker compose up
```

From your host machine, the simulated email API is available at:

```text
http://localhost:3100
```

From Bionic and other containers on the Docker Compose network, use:

```text
http://eval-mocks:3100
```

## Add the Integration to Bionic

Create a new OpenAPI integration in Bionic using the downloaded OpenAPI spec.
For the Docker Compose lab, keep the first server URL as:

```text
http://eval-mocks:3100
```

This first simulated integration does not require authentication. That keeps the
evaluation focused on whether the model can discover tools, read enterprise
context, and draft a useful response.

## Test Prompt

Once the integration is available, try:

```text
Check the RFX emails, identify the latest customer request, draft a reply asking
for missing security requirements, and do not send it yet.
```

A good result should:

1. List or inspect the available emails.
2. Read the CEO request.
3. Notice the security follow-up.
4. Create a draft response.
5. Avoid calling `sendDraft`.

That gives us a repeatable enterprise evaluation: the same API, the same data,
and the same expected behaviour every time we test the platform.

## Adding More Mock Integrations

The eval mocks image is intended to grow with the course. Add future simulated
systems under their own path prefix and OpenAPI spec under:

```text
infra-as-code/eval-mocks/openapi/
```

If the file should be downloadable from this lesson, also add a copy under the
course assets directory. Keeping the mock routes and OpenAPI specs in the repo
makes the enterprise evaluation repeatable in CI, local Docker Compose, and
shared demos.
