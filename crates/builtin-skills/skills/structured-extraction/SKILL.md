# Structured Extraction

Use this skill when a task depends on facts contained in uploaded documents or other files and those facts must be extracted before analysis.

## Workflow

1. List `/home/user/attachments` and identify every relevant source file and its format.
2. List `/home/user/functions`, then read the relevant `.md` catalogue file. Use the documented function name and parameters directly through `run_bash`.
3. Use the document conversion function for PDF, Word, Excel, and other supported formats. Pass uploaded files using their `/home/user/attachments/...` paths as documented by the function.
4. Extract each source separately. Preserve the source filename and every available location marker.
5. Return a compact structured representation of the extracted evidence before interpreting it.
6. If conversion fails or a format is unsupported, report the exact failure and do not substitute guessed content.

## Quality checks

- Every extracted claim has a source filename and location when available.
- Tables retain their headers and row or column meaning.
- Empty, unreadable, or missing content is marked as unavailable.
- Extraction and interpretation remain separate.
