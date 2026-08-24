-- migrate:up

WITH skill AS (
    INSERT INTO context.skills (
        name,
        description,
        visibility,
        is_system
    )
    VALUES (
        'document-coauthoring',
        'Guide users through structured co-authoring of proposals, specifications, RFCs, and decision documents, producing editable Typst source and a compiled PDF.',
        'Company',
        true
    )
    RETURNING id
)
INSERT INTO context.skill_files (
    skill_id,
    relative_path,
    contents
)
SELECT
    id,
    'SKILL.md',
    convert_to($$# Document Co-authoring

Use this skill when the user wants to write or revise a proposal, technical
specification, RFC, decision record, design document, or similar structured
documentation.

## Workflow

1. Establish the document's purpose, audience, scope, decision criteria,
   constraints, and desired outcome. Ask focused questions when important
   information is missing.
2. Propose a concise outline before drafting a substantial document.
3. Draft the document in Typst, keeping decisions, assumptions, alternatives,
   risks, evidence, and follow-up actions distinct.
4. Revise the draft from the user's feedback. Preserve the editable source so
   the document can be updated without rebuilding it from memory.
5. Compile the document using the Typst function documented in
   `/home/user/functions/typst.md`.

## Files

Store the working document at:

`/home/user/output/<document-name>/main.typ`

Use a short lowercase directory name. The Typst function accepts the source
through its VFS file-path parameter. It may read from `/home/user/output` as
well as uploaded attachments.

The compilation result is persisted in the same output directory. Keep the
Typst source and the compiled PDF together and report both paths to the user.

## Quality

- Ground claims in user-provided or retrieved evidence.
- Mark assumptions explicitly instead of presenting them as facts.
- Explain alternatives and why the recommendation was selected.
- Keep the document appropriate for its stated audience.
- Use clear headings, short sections, tables where useful, and consistent
  terminology.
- Do not claim that a document is complete until it has been compiled
  successfully.

The goal is a reviewable document and an editable source, not a one-shot wall
of text.
$$, 'UTF8')
FROM skill;

-- migrate:down

DELETE FROM context.skills
WHERE is_system = true
AND name = 'document-coauthoring';
