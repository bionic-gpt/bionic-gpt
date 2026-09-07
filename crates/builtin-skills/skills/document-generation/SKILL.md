---
name: document-generation
description: Create polished printable documents and PDFs, including forms, checklists, reports, briefs, comparisons, worksheets, task lists, and other operational documents. Use Typst and compile to PDF.
---
# Document Generation

Use this skill when the user asks for a printable document or PDF artifact.

## Workflow

1. Read all source material and identify:
   - required content
   - information users must fill in
   - approvals or signatures required
   - operational sequence or sections
2. Choose a layout based on how the document will actually be used.
3. Start from the closest known-good reference in `references/` rather than inventing Typst structure from scratch.
4. Write the Typst source to `/home/user/output/<document-name>/main.typ`.
5. Compile using the function documented in `/home/user/functions/typst.md`.
6. If compilation fails, fix the Typst error and recompile until successful.
7. Return the PDF only after successful compilation.

## References

Choose the closest reference and adapt it:

- `references/operational-form.typ` — forms, checklists, task lists, sign-offs, handwritten fields.
- `references/professional-report.typ` — reports with title, summary, sections, findings, tables, and recommendations.
- `references/comparison-recommendation.typ` — side-by-side option or vendor comparisons with criteria and a recommendation.

Use references as working Typst patterns, not as fixed visual designs. Preserve their known-good structure where practical and change content, labels, sections, and layout to fit the task.

## Design Principles

Treat the document as an operational artifact, not just formatted text.

- Preserve all required source content.
- Infer useful fields from the workflow when they are clearly implied, such as date, location, prepared by, check time, assigned employee, initials, notes, or verification.
- Use clear visual hierarchy: title, metadata, sections, task groups, verification, final approval.
- Prefer tables for repeated tasks and structured data entry.
- Give handwriting fields enough physical space.
- Make task descriptions the widest column.
- Keep initials and signature fields narrow but usable.
- Use page breaks deliberately when a section is easier to use on its own page.
- Keep final manager approval at the end when requested.
- Use concise completion-standard wording where appropriate without changing the underlying requirement.
- Add small workflow aids when they clearly improve usability, such as task IDs, section labels, or instructions.
- Do not add unsupported policies, business rules, names, dates, or requirements.

## Source Fidelity

When the task is based on attached material:

- Treat the source as authoritative for required content and terminology unless the user asks for rewriting.
- Do not omit required source items to make the document shorter or prettier.
- You may improve grouping, labels, typography, field design, and concise task wording without changing the underlying requirement.

## Typst Guidance

Typst is not Markdown.

Use Typst headings:

```typst
= Document Title
== Section Heading
```

For documents that resemble one of the references, adapt that reference instead of inventing layout primitives.

Avoid unsupported or untested Typst constructs. Prefer primitives already demonstrated in the references.

## Compilation

Read `/home/user/functions/typst.md` for the current compilation API.

Call the provided function directly with `run_python`; do not import it as a Python module.

Example:

```python
print(typst_compiledocument(**{
    'file_paths': ['/home/user/output/<document-name>/main.typ']
}))
```

## Repair Loop

If compilation fails:

1. Read the first compiler error carefully.
2. Fix the relevant Typst syntax or layout issue in `main.typ`.
3. Compile again.
4. Repeat until successful.

Do not switch to Markdown after a Typst error. Do not hide or ignore compilation failures.

## Quality Check

Before completing the task, verify:

- every required source item is present
- writable fields are large enough to use
- signatures and approvals are in the correct place
- the layout is practical when printed
- sections are not awkwardly orphaned across pages
- the PDF compiled successfully
