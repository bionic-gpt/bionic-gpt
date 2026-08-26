# Agentic RAG Introduction

Retrieval Augmented Generation pairs embeddings with grounded text generation. We detail each layer: chunking, storage, retrieval, and fusion into the final model response.

Best practices cover chunk sizes, metadata filters, and how to justify every generated sentence with citations in the UI.

## Tooling at a Glance

- `list_datasets`: Reads the prompt’s scope and returns every dataset the model is allowed to query so it never guesses dataset IDs and always stays inside the tenant’s permissions.
- `list_dataset_files`: Given a `dataset_id`, this function enumerates the actual files, sizes, and batch counts inside that dataset so the agent can pick the right sources before searching.
- Uploaded conversation files are available in the Bashkit VFS under `/home/user/attachments`; agents can inspect the directory and its manifest directly alongside curated datasets.

## How Agentic RAG Uses These Tools

1. The model starts by calling `list_datasets` to understand which curated knowledge bases are attached to the user’s prompt.
2. For any dataset that sounds relevant, it uses `list_dataset_files` to preview specific files and narrow retrieval to the ones that best match the question.
3. If the user uploaded fresh context, the agent inspects `/home/user/attachments` in the Bashkit VFS to cross-check the conversation attachments.
4. Only after these grounding steps does the agent run the retrieval/query pipeline, blend the cited facts into a draft answer, and return citations that map back to the dataset files or conversation documents surfaced through the tools above.

## Example Prompt and Flow

**User prompt**: “I just uploaded the latest ‘Retail Banking FAQ’ PDF. Using that and any banking compliance datasets we already configured, draft a response explaining which mortgage programs are available to freelancers in Canada.”

1. The model receives the prompt, recognizes it needs grounded information, and triggers `list_datasets` to see which banking datasets are linked to the current configuration.
2. It notices a dataset called “Banking Compliance” and calls `list_dataset_files` with that `dataset_id` to inspect which files (e.g., `canada_mortgage_rules.md`) might contain freelancer eligibility details.
3. Because the user mentioned an uploaded PDF, the agent inspects `/home/user/attachments` to locate “Retail Banking FAQ” and reads it from the VFS.
4. Armed with dataset file metadata and the uploaded file in the VFS, the retrieval layer fetches the most relevant chunks, grounds the answer, and the language model drafts a response citing the specific dataset file and uploaded PDF when referencing each mortgage rule.
