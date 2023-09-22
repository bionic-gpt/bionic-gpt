+++
title = "Retrieval Augmented Generation (RAG) from the ground up"
date = 2023-09-21
draft=true

[extra]
main_image = "blog/rag-from-scratch/rag-llama.webp"
listing_image = "blog/rag-from-scratch/rag-llama.webp"
+++


## Introduction

Here we want to take an approach that doesn't use a framework to show how RAG works. To do that we'll show exactly what gets passed into the model.

## Running LLama2 7b locally

## Prompt Templates - What are they actually?

```
Context: {history} \n {context}
User: {question}
Answer:
```

```
[INST]<<SYS>>You are a helpful assistant, you will use the provided context to answer user questions. Read the given context before answering questions and think step by step. If you can not answer a user question based on the provided context, inform the user. Do not use any other information for answering user<</SYS>>
Context: {history}
{context}
User: {question}
[/INST]
```

```
The prompt below is a question to answer, a task to complete, or a conversation to respond to; decide which and write an appropriate response.
### Prompt:
{{.Input}}
### Response:
```

## What do we use as history?

## Let's add some context

## How batching works

## Generating embeddings