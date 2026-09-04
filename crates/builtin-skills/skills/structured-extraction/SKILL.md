---
name: structured-extraction
description: Extract structured, source-located evidence from uploaded documents with Xberg for document analysis and validation.
---
# Structured Extraction

Use this skill when a task depends on facts contained in uploaded documents or other files and those facts must be extracted before analysis.

## Workflow

1. List `/home/user/attachments` and identify every relevant source file.
2. For each source, call `document_conversion_api_extractdocument` directly with `run_python`. Pass the attachment path as `file_path`.
3. In the same Python call, serialize the complete result to `/home/user/work/<source-name>.json` instead of printing it. For example:

```python
import json

result = document_conversion_api_extractdocument(
    file_path="/home/user/attachments/<source-file>"
)
with open("/home/user/work/<source-file>.json", "w") as extracted:
    extracted.write(json.dumps(result, ensure_ascii=False, indent=2))
print("Saved extraction to /home/user/work/<source-file>.json")
```

4. Inspect or search the saved JSON with `read_file` or `run_bash` commands such as `grep`; it remains available across later turns without appearing as a generated file in the conversation.
5. Preserve the source filename and every available location marker when reporting evidence. Keep extraction separate from interpretation.
6. If conversion fails or a format is unsupported, report the exact failure and do not substitute guessed content.

Xberg supports many document families, including PDF, Word and other office documents, spreadsheets, presentations, ebooks, email, archives, HTML/XML, structured text, images with OCR, and audio or video transcription. Let the conversion response determine whether a particular file is supported.

## Quality checks

- Every extracted claim has a source filename and location when available.
- Tables retain their headers and row or column meaning.
- Empty, unreadable, or missing content is marked as unavailable.
- Extraction and interpretation remain separate.
