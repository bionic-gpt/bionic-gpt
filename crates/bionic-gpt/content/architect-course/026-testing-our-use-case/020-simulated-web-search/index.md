# Simulated Web Search

The email integration gives the model a business request. A research workflow
also needs a repeatable way to discover external sources without depending on
live search results changing between evaluations.

This simulated web search API returns ten curated, real-world URLs about how
European banks, supervisors, and policy makers are approaching sovereign
generative AI.

## Download the Spec

- [Download the OpenAPI spec](/architect-course/testing-our-use-case/web-search.openapi.yaml)

The OpenAPI spec defines one deterministic search endpoint. Bionic uses this
spec to expose the search API as a callable integration, while the mock API
returns the same ten source URLs every time.

## What the Simulated API Provides

| Operation | Method | Path | Purpose |
| --- | --- | --- | --- |
| `searchWeb` | `GET` | `/web/search` | Return curated search results |

Each result includes a title, URL, source, publication date, summary, and a
short explanation of why the source is relevant. The URLs point to real pages,
so a later URL-reading capability can inspect the underlying source material.

The result set includes sources from the ECB, ECB Banking Supervision, the
European Commission, La Banque Postale, Santander, BBVA, and ING Germany.

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

From your host machine, the simulated search API is available at:

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

This simulated integration does not require authentication. It is designed to
test whether the model can discover a research tool, collect credible sources,
and produce an executive artifact from structured results.

## Test Prompt

Once the integration is available, try:

```text
Research how European banks are approaching sovereign generative AI and prepare
a 5-slide briefing for the CEO.
```

A good result should:

1. Discover and call the simulated web search integration.
2. Use the ten returned URLs as source material.
3. Group findings into a small number of executive themes.
4. Distinguish bank examples from regulator and policy context.
5. Produce a concise five-slide briefing, ideally as a generated artifact.

This gives us a repeatable research evaluation: the same prompt, the same
search results, and the same expected workflow every time.
