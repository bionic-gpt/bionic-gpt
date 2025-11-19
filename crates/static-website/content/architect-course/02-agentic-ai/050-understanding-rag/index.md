# Agentic RAG Introduction

Retrieval Augmented Generation pairs embeddings with grounded text generation. We detail each layer: chunking, storage, retrieval, and fusion into the final assistant response.

Best practices cover chunk sizes, metadata filters, and how to justify every generated sentence with citations in the UI.

## Tooling at a Glance

- `list_datasets`: Reads the prompt’s scope and returns every dataset the assistant is allowed to query so the model never guesses dataset IDs and always stays inside the tenant’s permissions.
- `list_dataset_files`: Given a `dataset_id`, this function enumerates the actual files, sizes, and batch counts inside that dataset so the agent can pick the right sources before searching.
- `list_documents`: This call enumerates ad-hoc documents shared in the current conversation (with real `file_id` values) so the agent can reference uploaded material alongside curated datasets.

## How Agentic RAG Uses These Tools

1. The assistant starts by calling `list_datasets` to understand which curated knowledge bases are attached to the user’s prompt.
2. For any dataset that sounds relevant, it uses `list_dataset_files` to preview specific files and narrow retrieval to the ones that best match the question.
3. If the user uploaded fresh context, `list_documents` provides the temporary file IDs so the agent can cross-check the conversation attachments.
4. Only after these grounding steps does the agent run the retrieval/query pipeline, blend the cited facts into a draft answer, and return citations that map back to the dataset files or conversation documents surfaced through the tools above.

## Example Prompt and Flow

**User prompt**: “I just uploaded the latest ‘Retail Banking FAQ’ PDF. Using that and any banking compliance datasets we already configured, draft a response explaining which mortgage programs are available to freelancers in Canada.”

1. The model receives the prompt, recognizes it needs grounded information, and triggers `list_datasets` to see which banking datasets are linked to the assistant.
2. It notices a dataset called “Banking Compliance” and calls `list_dataset_files` with that `dataset_id` to inspect which files (e.g., `canada_mortgage_rules.md`) might contain freelancer eligibility details.
3. Because the user mentioned an uploaded PDF, the agent calls `list_documents` to obtain the temporary `file_id` for “Retail Banking FAQ” so it can be read later (via `read_document`).
4. Armed with dataset file metadata and the ad-hoc document ID, the retrieval layer fetches the most relevant chunks, grounds the answer, and the language model drafts a response citing the specific dataset file and the uploaded PDF when referencing each mortgage rule.
