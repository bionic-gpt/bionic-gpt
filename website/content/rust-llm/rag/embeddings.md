+++
title = "Storing Embeddings"
description = "Embeddings"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 80
sort_by = "weight"


[extra]
toc = true
top = false
section = "rust-llm"
+++

> A text embedding is a compressed, abstract representation of text data where text of arbitrary length can be represented as a vector of numbers. Think of them as a universal encoding for text, where similar items are close to each other while dissimilar items are farther apart. [https://eugeneyan.com/writing/llm-patterns/](https://eugeneyan.com/writing/llm-patterns/)

![Architecture](../embedding-animation.gif)

## Overview

We have an API already to turn documents into text and we have an API to convert text into embeddings. Let's combine those together and store the results in Postgres.

We've set up the [Local AI](https://github.com/go-skynet/LocalAI) server already in the `devcontainer` the type of embeddings we are using are [text-embedding-ada-002](https://openai.com/blog/new-and-improved-embedding-model) from OpenAI.

## Testing the API

```sh
curl http://llm-api:8080/v1/embeddings \
  -H "Content-Type: application/json" \
  -d '{
    "input": "The food was delicious and the waiter...",
    "model": "text-embedding-ada-002"
  }'
```

## Batches

Models have a limit on the prompt size. So it makes sense to split the documents into 512 character chunks and generate embeddings based on those chunks.

So in the database a large document will be split into many smaller entries in the database.

## Consequences

- Do we need to split documents on boundaries i.e. chapter or heading?
- Cornucopia doesn't work with types it doesn't know about we can try pgvector = { version = "0.2", features = ["postgres"] }
- Looks like the dimension returned by `http://llm-api:8080/v1/embeddings` is 384 which has to match the settings in the DB
