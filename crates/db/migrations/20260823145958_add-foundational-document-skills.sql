-- migrate:up
WITH skill AS (
    INSERT INTO context.skills (name, description, visibility, is_system)
    VALUES (
        'structured-extraction',
        'Extract structured, source-located evidence from uploaded documents using the runtime document APIs.',
        'Company', true
    )
    RETURNING id
)
INSERT INTO context.skill_files (skill_id, relative_path, contents)
SELECT id, 'SKILL.md', convert_to($skill$
# Structured Extraction

Use this skill when a task depends on facts contained in uploaded documents or
other files and those facts must be extracted before analysis.

## Workflow

1. List `/home/user/attachments` and identify every relevant source file and its format.
2. List `/home/user/functions`, then read the relevant `.md` catalogue file. Use the documented function name and parameters directly through `run_bash`.
3. Use the document conversion function for PDF, Word, Excel, and other supported formats. Pass uploaded files using their `/home/user/attachments/...` paths as documented by the function.
4. Extract each source separately. Preserve the source filename and every available location marker, including PDF page, document heading or paragraph, worksheet name, cell or range, and table row or column.
5. Return a compact structured representation of the extracted evidence before interpreting it. Keep text from different source documents separate.
6. If conversion fails or a format is unsupported, report the exact failure and do not substitute guessed or typical content.

## Quality checks

- Every extracted claim has a source filename and location when available.
- Tables retain their headers and row or column meaning.
- Empty, unreadable, or missing content is marked as unavailable.
- Extraction and interpretation remain separate so later reasoning can be audited against the source material.
$skill$, 'UTF8')
FROM skill;

WITH skill AS (
    INSERT INTO context.skills (name, description, visibility, is_system)
    VALUES (
        'document-comparison',
        'Compare extracted documents against rubrics and reference documents with traceable evidence and explicit gap statuses.',
        'Company', true
    )
    RETURNING id
)
INSERT INTO context.skill_files (skill_id, relative_path, contents)
SELECT id, 'SKILL.md', convert_to($skill$
# Document Comparison

Use this skill when a user asks to compare documents, validate a document against a rubric or policy, reconcile evidence across files, or identify gaps and risks.

## Workflow

1. Identify the governing rubric, checklist, policy, contract, or reference document and distinguish it from the evidence documents.
2. Read `/home/user/skills/structured-extraction/SKILL.md` and use that workflow to extract every relevant source before drawing conclusions.
3. Build a requirement-by-requirement matrix. Include the requirement text, status, supporting document, precise location, and action needed for any gap.
4. Use exactly these statuses: **Pass** (explicit evidence satisfies the requirement), **Fail** (explicit evidence contradicts it), **Partial** (evidence satisfies only part or falls short of its threshold), and **Unknown** (the supplied documents do not establish it).
5. Reconcile conflicts between documents explicitly. Keep source facts, conclusions, and recommendations in separate sections.
6. Provide an executive recommendation that follows from the matrix, identify material risks and anomalies, and list targeted follow-up questions.

## Quality checks

- Every governing requirement appears exactly once in the matrix.
- Every Pass, Fail, or Partial result cites explicit evidence and a location.
- An omitted clause is Unknown unless another document supplies explicit evidence; absence is not proof of failure.
- Conflicting documents are called out rather than silently resolved.
- Do not invent legal, contractual, policy, or operational terms.
$skill$, 'UTF8')
FROM skill;

-- migrate:down
DELETE FROM context.skills
WHERE is_system = true
AND name IN ('structured-extraction', 'document-comparison');
