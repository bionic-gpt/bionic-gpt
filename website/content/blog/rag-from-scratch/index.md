+++
title = "Retrieval Augmented Generation (RAG) from the ground up"
date = 2023-09-21
draft=true

[extra]
main_image = "blog/rag-from-scratch/rag-llama.webp"
listing_image = "blog/rag-from-scratch/rag-llama.webp"
+++


## What is Retrieval Augmented Generation?

Basically we give the Large Language Model the best chance of answering the question by giving the model the answer as well as the question.

What?

Bare with us, we're going to show how we can use documents you already have to make this happen automagically.

## The very simplest RAG prompt

In this example we'll use a small language model at 7B parameters so you can follow along.

We'll remove all the frameworks and fluff so you can see exactly what gets sent to the model.

Run up a large language model API on your machine. (Assuming you have docker installed).


```sh
docker run -p 8080:8080 -it --rm ghcr.io/purton-tech/bionicgpt-model-api
```

Let's ask our model a question it probably doesn't know the answer to.

```sh
curl http://localhost:8080/v1/completions \
    -H "Content-Type: application/json" \
    -d '{"model": "ggml-gpt4all-j", "prompt": "What is a Dobblenobble?" }'
```

And one possible response is below. Note: you might get a different response due to something called temperature.

```sh
{"object":"text_completion","model":"ggml-gpt4all-j","choices":[{"index":0,"finish_reason":"stop","text":"As an AI language model, I do not have any information about the term \"Dobblenobblê\". It may be a misspelling or an outdated term. Can you please provide more context or information about what it means?"}],"usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0}}
```

Let me put format that for you.

> As an AI language model, I do not have any information about the term \"Dobblenobblê\". It may be a misspelling or an outdated term. Can you please provide more context or information about what it means?

Next we'll ask the same question and give the LLM the answer. But first a small diversion, we need to do some prompt engineering.

### The Question

```sh
curl http://localhost:8080/v1/completions \
    -H "Content-Type: application/json" \
    -d '{"model": "ggml-gpt4all-j", "prompt": "A Dobblenobble is a type of biscuit. What is a Dobblenobble?" }'
```

### The Answer - Welsh Biscuits

```json
{"object":"text_completion","model":"ggml-gpt4all-j","choices":[{"index":0,"finish_reason":"stop","text":"A dobbler nobble is a type of biscuit that originated in Wales and is popular among Welsh people. It typically consists of oatmeal, sugar and butter flavoured with orange or lemon zest, and is often served with milk or a dollop of jam. The name \"dobbler nobble\" is believed to have originated from the fact that it was originally baked in a muffin tin, which resembled the shape of an old-fashioned dobbler, a type of small animal found in Welsh folklore."}],"usage":{"prompt_tokens":0,"completion_tokens":0,"total_tokens":0}}
```

> A dobbler nobble is a type of biscuit that originated in Wales and is popular among Welsh people. It typically consists of oatmeal, sugar and butter flavoured with orange or lemon zest, and is often served with milk or a dollop of jam. The name \"dobbler nobble\" is believed to have originated from the fact that it was originally baked in a muffin tin, which resembled the shape of an old-fashioned dobbler, a type of small animal found in Welsh folklore.

I did not make this up. These are the answers I got

### Hallucinating

So we gave the model the answer and it kind of fed it back to us but then completely went off on one. In the trade that's called hallucinating.

As far as I know

* A dobbler nobble is not a welsh biscuit. (although it sounds like it would taste delicious)
* There's no small animal in Wales called a dobbler.

## A tiny amount of Prompt Engineering

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

## Let's stop the hallucinating (hopefully)

## The retrieval part of things

The generally agreed approach, and by that I mean

## Chunking up PDF's

## Storing them in database

## Adding a vector column

## Generating embeddings

## What is RAG? - A final definition

> Retrieval-Augmented Generation (RAG) fetches relevant data from outside the foundation model and enhances the input with this data, providing richer context to improve output.