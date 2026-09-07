---
name: dataset-analysis
description: Use assistant datasets for grounded answers with rag-search and rag-read.
---
# Dataset Analysis

Use this skill when the user asks a question that should be answered from the assistant's datasets.

1. Inspect `/home/user/datasets/index.json` to see available datasets.
2. Use `rag-search "query" --limit N` to find relevant chunks.
3. Read returned chunk paths with `rag-read PATH` or `cat PATH`.
4. Answer from the retrieved evidence. Say when the datasets do not contain enough information.
