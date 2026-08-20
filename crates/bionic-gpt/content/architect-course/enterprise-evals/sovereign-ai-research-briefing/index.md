# Sovereign AI Research Briefing

This eval tests whether the model can discover a research integration, use
curated source results, separate bank examples from policy and regulator
context, and produce a concise executive briefing.

A research workflow needs a repeatable way to discover external sources without
depending on live search results changing between evaluations.

The research API returns ten curated, real-world URLs about how European banks,
supervisors, and policy makers are approaching sovereign generative AI.

## Test Prompt

Once the integration is available, try:

```text
Research how European banks are approaching sovereign generative AI and prepare
a 5-slide executive briefing.
```

A good result should:

1. Discover and call the research integration.
2. Use the ten returned URLs as source material.
3. Group findings into a small number of executive themes.
4. Distinguish bank examples from regulator and policy context.
5. Produce a concise five-slide briefing, ideally as a generated artifact.

## Download the Spec

- [Download the OpenAPI spec](/architect-course/enterprise-evals/web-search.openapi.yaml)

The OpenAPI spec defines one deterministic search endpoint. Bionic uses this
spec to expose the search API as a callable integration, while the mock API
returns the same ten source URLs every time.

## What the Eval API Provides

| Operation | Method | Path | Purpose |
| --- | --- | --- | --- |
| `searchWeb` | `GET` | `/web/search` | Return curated search results |

Each result includes a title, URL, source, publication date, summary, and a
short explanation of why the source is relevant. The URLs point to real pages,
so a later URL-reading capability can inspect the underlying source material.

The result set includes sources from the ECB, ECB Banking Supervision, the
European Commission, La Banque Postale, Santander, BBVA, and ING Germany.

## Add the Integration to Bionic

Download the OpenAPI spec, then go to the admin area in Bionic. Open
**OpenAPI Specs**, add a new spec, paste or upload the research eval YAML, and
specify it as `websearch`.

Return to the app, open **Integrations**, add an integration, and choose the
web search spec you just added.

This eval integration does not require authentication. It is designed to test
whether the model can discover a research tool, collect credible sources, and
produce an executive artifact from structured results.

This gives us a repeatable research evaluation: the same prompt, the same
search results, and the same expected workflow every time.
