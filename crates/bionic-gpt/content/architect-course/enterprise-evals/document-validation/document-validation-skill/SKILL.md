---
name: document-validation
description: Validate uploaded business documents against a supplied rubric and produce an evidence-backed Pass, Fail, Partial, or Unknown assessment.
---

# Document Validation

Use this skill when a user asks whether one or more uploaded documents satisfy a
policy, checklist, standard, contract rubric, or acceptance criteria.

## Workflow

1. Inspect `/home/user/attachments` and identify every source document and its
   format. Treat the user's rubric or checklist as the governing requirements.
2. Read `/home/user/skills/document-validation/rubric.md` when the packaged
   rubric is the applicable reference. If the user supplied a different rubric,
   use that instead.
3. Use the configured document-extraction integration when conversion is
   needed. Extract each source separately and preserve document names, pages,
   headings, sheet names, and cell/range locations where available.
4. Build a requirement matrix before writing conclusions. For each requirement,
   record the status, exact supporting evidence, source location, and any gap.
5. Use only four statuses:
   - **Pass**: explicit evidence satisfies the requirement.
   - **Fail**: explicit evidence contradicts the requirement.
   - **Partial**: evidence satisfies only part of the requirement or is weaker
     than the rubric threshold.
   - **Unknown**: the supplied documents do not establish the requirement.
6. Keep contractual or policy facts separate from recommendations. Never turn
   an absent clause into an asserted obligation.
7. Write the report to `/home/user/output/document-validation/report.md` when
   the user asks for a saved artifact. Include an executive recommendation,
   the full matrix, material risks, and specific follow-up questions.

## Quality checks

- Every rubric requirement appears exactly once in the matrix.
- Every Pass, Fail, or Partial result has document evidence and a location.
- Unknown means “not established by the supplied documents,” not “probably
  false.”
- Conflicts between documents are called out explicitly.
- The recommendation follows from the matrix and does not invent legal advice.
