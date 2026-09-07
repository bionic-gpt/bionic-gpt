---
name: document-generation
description: Create polished printable documents and PDFs, including forms, checklists, reports, letters, worksheets, task lists, and similar artifacts, using Typst and the document compilation function.
---
# Document Generation

Use this skill whenever the user asks for a printable document or PDF artifact, including forms, checklists, reports, letters, worksheets, task lists, reference sheets, or similar documents.

## Workflow

1. Read any attached source material first and preserve its required content, structure, terminology, and constraints.
2. Determine the document type and choose a layout that makes the result practical to use, not merely visually attractive.
3. Write valid Typst source to `/home/user/output/<document-name>/main.typ`.
4. Compile the document using the Typst compilation function documented in `/home/user/functions/typst.md`.
5. If compilation fails, read the compiler error, repair the Typst source, and compile again. Repeat until compilation succeeds.
6. Return the generated PDF only after successful compilation. Keep the editable `main.typ` source alongside it.

## Typst Rules

Typst is not Markdown. Write Typst syntax directly.

Use:

```typst
= Document Title
== Section Heading
```

Do not use Markdown headings such as:

```text
# Document Title
## Section Heading
```

Do not create blank form fields by typing long runs of underscores such as `________________`. In Typst, underscores have markup meaning and can cause parse errors. Use layout primitives such as `line`, `box`, `table`, or empty cells instead.

For example:

```typst
#set page(margin: 18mm)
#set text(size: 10pt)

= Daily Task List

#table(
  columns: (3fr, 1.3fr, 0.8fr, 2fr),
  inset: 5pt,
  stroke: 0.5pt,
  [*Task*], [*Assigned To*], [*Initials*], [*Notes*],
  [Turn on all demos and verify operation.], [], [], [],
)
```

For writable fields, prefer table cells or visible rules:

```typst
Manager Name: #line(length: 55mm)
Date: #line(length: 35mm)
Signature: #line(length: 55mm)
```

## PDF and Form Quality

When creating operational forms, checklists, or worksheets:

- Prefer tables when users must repeatedly enter names, initials, signatures, statuses, or notes.
- Preserve enough whitespace for handwriting.
- Keep column widths practical: task descriptions should receive the most space, initials the least.
- Use clear section headings and visual separation between sections.
- Keep manager approval or final sign-off exactly where the user requested it.
- Avoid decorative elements that reduce writing space or legibility.
- Fit related content together where practical; avoid awkward single-row page breaks.
- Never omit requested source content simply to make the document shorter.

## Source Fidelity

When the task is based on an attached document:

- Treat the attachment as authoritative for required tasks and wording unless the user asks for rewriting.
- Do not invent missing duties, policies, names, dates, or business rules.
- You may improve layout, grouping, typography, labels, and field design without changing the underlying requirements.

## Compilation

Read `/home/user/functions/typst.md` for the current compilation API. Call the provided function directly by name with `run_python`; do not import it as a Python module.

Typical invocation:

```python
print(typst_compiledocument({
    'file_paths': ['/home/user/output/<document-name>/main.typ']
}))
```

Do not use:

```python
from typst import typst_compiledocument
```

The function is provided by the runtime rather than as an importable Python package.

## Repair Loop

If Typst compilation fails:

1. Read the first compiler error carefully.
2. Fix the relevant Typst syntax or layout issue in `main.typ`.
3. Compile again.
4. Repeat until successful.

Do not switch the source file to Markdown after a Typst error. Do not hide or ignore compilation failures.

## Completion Criteria

A document task is complete only when:

- all user-required content is present,
- requested fields and sign-off areas are usable,
- the Typst source compiles successfully,
- and the generated PDF exists.
