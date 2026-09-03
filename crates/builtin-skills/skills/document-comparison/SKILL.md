# Document Comparison

Use this skill when a user asks to compare documents, validate a document against a rubric or policy, reconcile evidence across files, or identify gaps and risks.

## Workflow

1. Identify the governing rubric, checklist, policy, contract, or reference document and distinguish it from the evidence documents.
2. Read `/home/user/skills/structured-extraction/SKILL.md` and use that workflow to extract every relevant source before drawing conclusions.
3. Build a requirement-by-requirement matrix with the requirement, status, supporting document, precise location, and action needed.
4. Use exactly these statuses: **Pass**, **Fail**, **Partial**, and **Unknown**.
5. Reconcile conflicts explicitly. Keep source facts, conclusions, and recommendations separate.

## Quality checks

- Every governing requirement appears exactly once in the matrix.
- Every Pass, Fail, or Partial result cites explicit evidence and a location.
- An omitted clause is Unknown unless another document supplies explicit evidence.
- Conflicting documents are called out rather than silently resolved.
