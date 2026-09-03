# Document Co-authoring

Use this skill when the user wants to write or revise a proposal, technical specification, RFC, decision record, design document, or similar structured documentation.

## Workflow

1. Establish the document's purpose, audience, scope, decision criteria, constraints, and desired outcome.
2. Propose a concise outline before drafting a substantial document.
3. Draft the document in the document-generation format, keeping decisions, assumptions, alternatives, risks, evidence, and follow-up actions distinct.
4. Revise the draft from the user's feedback. Preserve the editable source.
5. Compile the document using the document-generation function documented in `/home/user/functions`.

Store the working document at `/home/user/output/<document-name>/main.typ`. Keep the editable source and compiled PDF together. Describe them as the editable source document and generated PDF.

## Quality

- Ground claims in user-provided or retrieved evidence.
- Mark assumptions explicitly.
- Explain alternatives and why the recommendation was selected.
- Do not claim that a document is complete until it has compiled successfully.
