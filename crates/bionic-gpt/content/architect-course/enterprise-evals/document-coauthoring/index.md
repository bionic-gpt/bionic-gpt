# Document Co-authoring

This eval tests whether the model can turn an ambiguous documentation request
into a structured, reviewable document. It combines skill discovery,
clarifying questions, iterative drafting, Typst compilation, and persistent
artifact generation.

The document co-authoring skill and Typst integration are system capabilities.
They should already be available after the database migrations have run.

## What this eval tests

- Does the model discover and read the document co-authoring skill?
- Does it establish the audience, scope, constraints, and decision criteria?
- Does it separate evidence, assumptions, alternatives, risks, and decisions?
- Does it create editable Typst source rather than only returning chat text?
- Does it compile and persist a PDF artifact?
- Can the user continue revising the same source document?

## Test Prompt

```text
Help me co-author a technical decision record for introducing a private,
self-hosted document-processing capability for our internal AI platform.

Start by asking the questions needed to establish the audience, scope,
decision criteria, constraints, alternatives, risks, and recommendation.
Then create an editable Typst source document and a compiled PDF. Keep the
document suitable for review by engineering leadership.
```

## Expected workflow

1. The model discovers `document-coauthoring` under
   `/home/user/skills` and reads its instructions.
2. It lists `/home/user/functions` and reads the Typst function catalogue
   before attempting compilation.
3. It asks focused questions before committing to a detailed structure.
4. It writes the draft to a path such as:
   `/home/user/output/document-processing-decision/main.typ`.
5. It calls the Typst compilation function with that VFS path.
6. The compiled PDF appears in the same output directory and is shown as a
   generated artifact in the conversation.

## What to evaluate

The strongest result is not merely a polished document. It is a repeatable
workflow in which the model makes uncertainty visible, keeps the source
editable, compiles the artifact, and responds cleanly to requests such as:

```text
Add a section comparing the self-hosted option with an approved hosted API.
Make the recommendation conditional on data residency and operational
ownership, then compile the revised document again.
```

Inspect both the `.typ` source and the PDF. Confirm that the second request
updates the existing document rather than creating an unrelated answer in
chat.
