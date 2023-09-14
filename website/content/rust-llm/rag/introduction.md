+++
title = "Introduction"
description = "Embeddings"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

Retrieval-Augmented Generation (RAG) fetches relevant data from outside the foundation model and enhances the input with this data, providing richer context to improve output.

## Why RAG?

RAG helps reduce hallucination by grounding the model on the retrieved context, thus increasing factuality. In addition, it’s cheaper to keep retrieval indices up-to-date than to continuously pre-train an LLM. This cost efficiency makes it easier to provide LLMs with access to recent data via RAG. Finally, if we need to update or remove data such as biased or toxic documents, it’s more straightforward to update the retrieval index.

In short, RAG applies mature and simpler ideas from the field of information retrieval to support LLM generation. In a recent Sequoia survey, 88% of respondents believe that retrieval will be a key component of their stack.

## Architecture

![Architecture](../embeddings.png)